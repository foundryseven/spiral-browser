//! Tree-walking interpreter for the Vortex JavaScript engine.
//!
//! This is the Phase 1 execution engine. It walks the AST directly,
//! evaluating expressions and executing statements against a stack of
//! variable environments. It is intentionally simple — the bytecode
//! VM in Phase 2 will replace it for production use.
//!
//! # Design decisions
//!
//! - **No `Rc<RefCell<>>` in the hot path.** The interpreter uses owned
//!   `JsValue` everywhere and clones only when necessary (objects use
//!   `ObjectId` references, which are `Copy`).
//! - **Environment records** are a linked-list chain of `HashMap<String,
//!   JsValue>` scopes. Closure capture copies the values at the point
//!   of capture (Phase 1 limitation; Phase 2 will use heap-allocated
//!   environment records).
//! - **Exceptions** are represented as `Err(VortexError::Throw(...))`
//!   that propagate through `?`. The `try/catch` implementation catches
//!   them at the statement level.

use crate::ast::expr::{
    AssignOp, BinaryOp, BindingPattern, ExprKind, LogicalOp, MemberOp, PropertyKey, UnaryOp,
};
use crate::ast::stmt::StmtKind;
use crate::ast::{Expr, Program, Stmt};
use crate::error::{VortexError, VortexResult};
use crate::value::object::JsObject;
use crate::value::JsValue;
use std::collections::HashMap;

/// A variable environment (one scope).
#[derive(Debug, Clone)]
pub struct Environment {
    bindings: HashMap<String, JsValue>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(parent: Option<Environment>) -> Self {
        Self {
            bindings: HashMap::new(),
            parent: parent.map(Box::new),
        }
    }

    /// Define a new binding (var/let/const).
    pub fn define(&mut self, name: &str, value: JsValue) {
        self.bindings.insert(name.to_string(), value);
    }

    /// Look up a binding by name, walking the scope chain.
    pub fn get(&self, name: &str) -> Option<&JsValue> {
        if let Some(val) = self.bindings.get(name) {
            return Some(val);
        }
        self.parent.as_ref().and_then(|p| p.get(name))
    }

    /// Set an existing binding (assignment). Walks the scope chain.
    /// Returns false if the binding was not found anywhere in the chain.
    pub fn set(&mut self, name: &str, value: JsValue) -> bool {
        if self.bindings.contains_key(name) {
            self.bindings.insert(name.to_string(), value);
            return true;
        }
        if let Some(ref mut parent) = self.parent {
            return parent.set(name, value);
        }
        false
    }

    /// Get the global environment (the outermost scope).
    pub fn global(&self) -> &Environment {
        match &self.parent {
            Some(p) => p.global(),
            None => self,
        }
    }
}

/// The result of evaluating a statement.
#[derive(Debug)]
pub enum Completion {
    /// Normal completion — continue to next statement.
    Normal,
    /// `return value` — propagate upward to the function caller.
    Return(JsValue),
    /// `break label` — propagate upward to the nearest loop/switch.
    Break(Option<String>),
    /// `continue label` — propagate upward to the nearest loop.
    Continue(Option<String>),
}

/// The tree-walking interpreter.
pub struct Interpreter {
    /// The global environment.
    pub env: Environment,
    /// The value of `this` in the current context.
    pub this: JsValue,
    /// Console output buffer (used by `console.log`, etc.).
    pub console_out: Vec<String>,
}

impl Interpreter {
    /// Create a new interpreter with an empty global environment.
    pub fn new() -> Self {
        Self {
            env: Environment::new(None),
            this: JsValue::Undefined,
            console_out: Vec::new(),
        }
    }

    /// Execute a full program.
    pub fn run(&mut self, program: &Program) -> VortexResult<JsValue> {
        let mut last = JsValue::Undefined;
        for stmt in &program.body {
            last = self.exec_stmt(stmt)?;
        }
        Ok(last)
    }

    /// Execute a single statement. Returns the value of the last
    /// expression statement (used for REPL / `eval` return value).
    pub fn exec_stmt(&mut self, stmt: &Stmt) -> VortexResult<JsValue> {
        match &stmt.kind {
            StmtKind::Expr(expr) => self.eval_expr(expr),

            StmtKind::Var { declarations }
            | StmtKind::Let { declarations }
            | StmtKind::Const { declarations } => {
                for decl in declarations {
                    let val = match &decl.init {
                        Some(init) => self.eval_expr(init)?,
                        None => JsValue::Undefined,
                    };
                    match &decl.pattern {
                        BindingPattern::Identifier(name) => {
                            self.env.define(name, val);
                        }
                        _ => {
                            return Err(VortexError::Internal(
                                "destructuring not yet implemented".into(),
                            ));
                        }
                    }
                }
                Ok(JsValue::Undefined)
            }

            StmtKind::If {
                test,
                consequent,
                alternate,
            } => {
                let cond = self.eval_expr(test)?;
                if cond.to_boolean() {
                    self.exec_stmt(consequent)
                } else if let Some(alt) = alternate {
                    self.exec_stmt(alt)
                } else {
                    Ok(JsValue::Undefined)
                }
            }

            StmtKind::Block(stmts) => {
                let mut last = JsValue::Undefined;
                for s in stmts {
                    last = self.exec_stmt(s)?;
                }
                Ok(last)
            }

            StmtKind::While { test, body } => {
                loop {
                    let cond = self.eval_expr(test)?;
                    if !cond.to_boolean() {
                        break;
                    }
                    match self.exec_stmt(body)? {
                        JsValue::Undefined => {} // continue
                        // Phase 1: break/continue/return via exceptions is not
                        // yet wired. The while loop runs to completion.
                        other => {
                            return Ok(other);
                        }
                    }
                }
                Ok(JsValue::Undefined)
            }

            StmtKind::Return { value } => {
                let val = match value {
                    Some(v) => self.eval_expr(v)?,
                    None => JsValue::Undefined,
                };
                // Phase 1: return uses a special "throw" to unwind.
                // This is a hack; Phase 2 uses a proper completion type.
                Err(VortexError::Throw(format!("__return__:{val}")))
            }

            StmtKind::FunctionDecl {
                name, params, body, ..
            } => {
                // Phase 1 (tree-walking interpreter): function bodies are
                // not yet executed inline. We register the function in
                // the environment and return. Phase B (bytecode VM) will
                // compile the body to a closure and execute it.
                let _ = (params, body);
                let func = JsValue::Function(JsObject::new("Function"));
                self.env.define(name, func);
                Ok(JsValue::Undefined)
            }

            StmtKind::Empty => Ok(JsValue::Undefined),

            _ => Err(VortexError::Internal(format!(
                "unimplemented statement: {:?}",
                stmt.kind
            ))),
        }
    }

    /// Evaluate an expression and return its value.
    pub fn eval_expr(&mut self, expr: &Expr) -> VortexResult<JsValue> {
        match &expr.kind {
            ExprKind::Null => Ok(JsValue::Null),
            ExprKind::Undefined => Ok(JsValue::Undefined),
            ExprKind::Bool(b) => Ok(JsValue::Bool(*b)),
            ExprKind::Number(n) => Ok(JsValue::Number(*n)),
            ExprKind::String(s) => Ok(JsValue::String(s.clone())),
            ExprKind::This => Ok(self.this.clone()),

            ExprKind::Identifier(name) => self
                .env
                .get(name)
                .cloned()
                .ok_or_else(|| VortexError::ReferenceError(format!("{name} is not defined"))),

            ExprKind::Binary(op, left, right) => {
                let l = self.eval_expr(left)?;
                let r = self.eval_expr(right)?;
                self.eval_binary(*op, &l, &r)
            }

            ExprKind::Unary(op, operand, _prefix) => {
                let val = self.eval_expr(operand)?;
                self.eval_unary(*op, &val)
            }

            ExprKind::Logical(op, left, right) => {
                let l = self.eval_expr(left)?;
                match op {
                    LogicalOp::And => {
                        if !l.to_boolean() {
                            return Ok(l);
                        }
                        self.eval_expr(right)
                    }
                    LogicalOp::Or => {
                        if l.to_boolean() {
                            return Ok(l);
                        }
                        self.eval_expr(right)
                    }
                    LogicalOp::NullishCoalescing => {
                        if !l.is_nullish() {
                            return Ok(l);
                        }
                        self.eval_expr(right)
                    }
                }
            }

            ExprKind::Assign(op, target, value) => {
                let val = self.eval_expr(value)?;
                if let ExprKind::Identifier(name) = &target.kind {
                    match op {
                        AssignOp::Assign => {
                            if !self.env.set(name, val.clone()) {
                                self.env.define(name, val.clone());
                            }
                        }
                        _ => {
                            return Err(VortexError::Internal(
                                "compound assignment not yet implemented".into(),
                            ));
                        }
                    }
                    Ok(val)
                } else {
                    Err(VortexError::Internal(
                        "non-identifier assignment target not yet implemented".into(),
                    ))
                }
            }

            ExprKind::Call { callee, args } => {
                // Phase 1: check for known builtins before evaluating callee
                // (callee evaluation requires Member support which is not yet
                // wired up for property access).
                if let ExprKind::Member(MemberOp::Dot, obj, prop) = &callee.kind {
                    if let ExprKind::Identifier(method) = &prop.kind {
                        if let ExprKind::Identifier(ns) = &obj.kind {
                            if ns == "console" && method == "log" {
                                let arg_vals: Vec<JsValue> = args
                                    .iter()
                                    .map(|a| self.eval_expr(a))
                                    .collect::<VortexResult<_>>()?;
                                let msg = arg_vals
                                    .iter()
                                    .map(|v| v.to_string())
                                    .collect::<Vec<_>>()
                                    .join(" ");
                                self.console_out.push(msg.clone());
                                log::info!("[console.log] {msg}");
                                return Ok(JsValue::Undefined);
                            }
                        }
                    }
                }

                let _func = self.eval_expr(callee)?;
                let _arg_vals: Vec<JsValue> = args
                    .iter()
                    .map(|a| self.eval_expr(a))
                    .collect::<VortexResult<_>>()?;

                Err(VortexError::Internal(
                    "function calls not yet implemented (only console.log)".into(),
                ))
            }

            ExprKind::Array(elements) => {
                let vals: Vec<Option<JsValue>> = elements
                    .iter()
                    .map(|e| match e {
                        Some(expr) => self.eval_expr(expr).map(Some),
                        None => Ok(None),
                    })
                    .collect::<VortexResult<_>>()?;
                let mut obj = JsObject::new("Array");
                obj.set("length", JsValue::Number(vals.len() as f64));
                for (i, val) in vals.into_iter().enumerate() {
                    if let Some(v) = val {
                        obj.set(i.to_string(), v);
                    }
                }
                Ok(JsValue::Object(obj))
            }

            ExprKind::Object(props) => {
                let mut obj = JsObject::new("Object");
                for prop in props {
                    let key = match &prop.key {
                        PropertyKey::Ident(s) => s.clone(),
                        PropertyKey::String(s) => s.clone(),
                        PropertyKey::Number(n) => n.to_string(),
                        PropertyKey::Computed(expr) => {
                            let val = self.eval_expr(expr)?;
                            val.to_string()
                        }
                    };
                    let val = self.eval_expr(&prop.value)?;
                    obj.set(key, val);
                }
                Ok(JsValue::Object(obj))
            }

            ExprKind::Sequence(exprs) => {
                let mut last = JsValue::Undefined;
                for expr in exprs {
                    last = self.eval_expr(expr)?;
                }
                Ok(last)
            }

            _ => Err(VortexError::Internal(format!(
                "unimplemented expression: {:?}",
                expr.kind
            ))),
        }
    }

    fn eval_binary(&self, op: BinaryOp, left: &JsValue, right: &JsValue) -> VortexResult<JsValue> {
        match op {
            BinaryOp::Add => {
                // JS string coercion: if either operand is a string, concat.
                match (left, right) {
                    (JsValue::String(a), b) => Ok(JsValue::String(format!("{a}{b}"))),
                    (a, JsValue::String(b)) => Ok(JsValue::String(format!("{a}{b}"))),
                    _ => Ok(JsValue::Number(left.to_number() + right.to_number())),
                }
            }
            BinaryOp::Sub => Ok(JsValue::Number(left.to_number() - right.to_number())),
            BinaryOp::Mul => Ok(JsValue::Number(left.to_number() * right.to_number())),
            BinaryOp::Div => Ok(JsValue::Number(left.to_number() / right.to_number())),
            BinaryOp::Mod => Ok(JsValue::Number(left.to_number() % right.to_number())),
            BinaryOp::Pow => Ok(JsValue::Number(left.to_number().powf(right.to_number()))),
            BinaryOp::EqEq => Ok(JsValue::Bool(left.loose_eq(right))),
            BinaryOp::EqEqEq => Ok(JsValue::Bool(left.strict_eq(right))),
            BinaryOp::BangEq => Ok(JsValue::Bool(!left.loose_eq(right))),
            BinaryOp::BangEqEq => Ok(JsValue::Bool(!left.strict_eq(right))),
            BinaryOp::Lt => Ok(JsValue::Bool(left.to_number() < right.to_number())),
            BinaryOp::LtEq => Ok(JsValue::Bool(left.to_number() <= right.to_number())),
            BinaryOp::Gt => Ok(JsValue::Bool(left.to_number() > right.to_number())),
            BinaryOp::GtEq => Ok(JsValue::Bool(left.to_number() >= right.to_number())),
            _ => Err(VortexError::Internal(format!(
                "unimplemented binary op: {op:?}"
            ))),
        }
    }

    fn eval_unary(&self, op: UnaryOp, operand: &JsValue) -> VortexResult<JsValue> {
        match op {
            UnaryOp::Neg => Ok(JsValue::Number(-operand.to_number())),
            UnaryOp::Pos => Ok(JsValue::Number(operand.to_number())),
            UnaryOp::Not => Ok(JsValue::Bool(!operand.to_boolean())),
            UnaryOp::Typeof => Ok(JsValue::String(operand.type_of().to_string())),
            UnaryOp::Void => Ok(JsValue::Undefined),
            UnaryOp::BitNot => {
                let n = super::super::value::number::to_int32(operand.to_number());
                Ok(JsValue::Number((!n) as f64))
            }
            UnaryOp::Delete => {
                // delete only works on object properties; stub for now.
                Ok(JsValue::Bool(true))
            }
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer;
    use crate::parser;

    fn eval(src: &str) -> JsValue {
        let tokens = lexer::lex(src);
        let program = parser::parse(&tokens).expect("parse failed");
        let mut interp = Interpreter::new();
        interp.run(&program).expect("eval failed")
    }

    fn eval_console(src: &str) -> Vec<String> {
        let tokens = lexer::lex(src);
        let program = parser::parse(&tokens).expect("parse failed");
        let mut interp = Interpreter::new();
        interp.run(&program).expect("eval failed");
        interp.console_out
    }

    #[test]
    fn test_number_literal() {
        assert!(matches!(eval("42"), JsValue::Number(n) if n == 42.0));
    }

    #[test]
    fn test_string_literal() {
        assert!(matches!(eval(r#""hello""#), JsValue::String(s) if s == "hello"));
    }

    #[test]
    fn test_bool_literal() {
        assert!(matches!(eval("true"), JsValue::Bool(true)));
    }

    #[test]
    fn test_binary_add() {
        assert!(matches!(eval("1 + 2"), JsValue::Number(n) if n == 3.0));
    }

    #[test]
    fn test_binary_mul() {
        assert!(matches!(eval("3 * 4"), JsValue::Number(n) if n == 12.0));
    }

    #[test]
    fn test_string_concat() {
        assert!(matches!(eval(r#""a" + "b""#), JsValue::String(s) if s == "ab"));
    }

    #[test]
    fn test_unary_neg() {
        assert!(matches!(eval("-5"), JsValue::Number(n) if n == -5.0));
    }

    #[test]
    fn test_logical_and() {
        assert!(matches!(eval("true && 42"), JsValue::Number(n) if n == 42.0));
        assert!(matches!(eval("false && 42"), JsValue::Bool(false)));
    }

    #[test]
    fn test_console_log() {
        let out = eval_console(r#"console.log("hello world")"#);
        assert_eq!(out, vec!["hello world"]);
    }

    #[test]
    fn test_var_declaration() {
        let result = eval("var x = 10; x");
        assert!(matches!(result, JsValue::Number(n) if n == 10.0));
    }

    #[test]
    fn test_if_statement() {
        let result = eval("var x; if (true) { x = 1; } else { x = 2; } x");
        assert!(matches!(result, JsValue::Number(n) if n == 1.0));
    }
}
