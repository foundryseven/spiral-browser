//! Expression AST nodes.

use super::stmt::Stmt;
use super::Span;

/// An expression node.
#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

/// The discriminant for expression variants.
#[derive(Debug, Clone)]
pub enum ExprKind {
    /// `null`
    Null,
    /// `undefined`
    Undefined,
    /// `true` / `false`
    Bool(bool),
    /// Numeric literal.
    Number(f64),
    /// String literal.
    String(String),
    /// Template literal (raw parts + interpolated expressions).
    Template { parts: Vec<TemplatePart> },
    /// `this`
    This,
    /// Identifier reference (`x`, `foo`).
    Identifier(String),
    /// Binary expression: `a + b`, `a === b`, etc.
    Binary(BinaryOp, Box<Expr>, Box<Expr>),
    /// Unary expression: `!x`, `-x`, `typeof x`, `void x`, `delete x.prop`.
    Unary(UnaryOp, Box<Expr>, PrefixOrPostfix),
    /// Logical expression: `a && b`, `a || b`, `a ?? b`.
    Logical(LogicalOp, Box<Expr>, Box<Expr>),
    /// Assignment: `a = b`, `a += b`, etc.
    Assign(AssignOp, Box<Expr>, Box<Expr>),
    /// Member access: `a.b` or `a[b]`.
    Member(MemberOp, Box<Expr>, Box<Expr>),
    /// Call expression: `f(a, b)`.
    Call { callee: Box<Expr>, args: Vec<Expr> },
    /// `new Foo(a, b)`.
    New { callee: Box<Expr>, args: Vec<Expr> },
    /// Conditional (ternary): `a ? b : c`.
    Conditional {
        test: Box<Expr>,
        consequent: Box<Expr>,
        alternate: Box<Expr>,
    },
    /// Array literal: `[1, 2, 3]`.
    Array(Vec<Option<Expr>>),
    /// Object literal: `{ a: 1, b: 2 }`.
    Object(Vec<Property>),
    /// Function expression: `function(a, b) { ... }`.
    Function {
        name: Option<String>,
        params: Vec<Param>,
        body: Vec<Stmt>,
        is_async: bool,
        is_generator: bool,
    },
    /// Arrow function: `(a, b) => expr` or `a => expr`.
    ArrowFunction {
        params: Vec<Param>,
        body: ArrowBody,
        is_async: bool,
    },
    /// Class expression.
    Class {
        name: Option<String>,
        super_class: Option<Box<Expr>>,
        methods: Vec<ClassMethod>,
    },
    /// Spread element: `...expr`.
    Spread(Box<Expr>),
    /// `yield expr`.
    Yield {
        value: Option<Box<Expr>>,
        delegate: bool,
    },
    /// `await expr`.
    Await(Box<Expr>),
    /// Sequence expression (comma operator): `(a, b, c)`.
    Sequence(Vec<Expr>),
    /// `void expr`.
    Void(Box<Expr>),
    /// `typeof expr` (as an expression, not the `typeof` unary op).
    Typeof(Box<Expr>),
    /// `delete expr`.
    Delete(Box<Expr>),
    /// `super.prop` or `super[expr]`.
    SuperMember(MemberOp, Box<Expr>),
    /// `super(args)`.
    SuperCall(Vec<Expr>),
    /// Tagged template literal: `` tag`hello` ``.
    TaggedTemplate {
        tag: Box<Expr>,
        parts: Vec<TemplatePart>,
    },
}

/// A part of a template literal — either a raw string or an interpolated
/// expression.
#[derive(Debug, Clone)]
pub enum TemplatePart {
    Raw(String),
    Expr(Expr),
}

/// Binary operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    EqEq,
    EqEqEq,
    BangEq,
    BangEqEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    In,
    Instanceof,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    UShr,
}

/// Unary operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Pos,
    Not,
    BitNot,
    Typeof,
    Void,
    Delete,
}

/// Whether a unary operator is prefix or postfix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefixOrPostfix {
    Prefix,
    Postfix,
}

/// Logical operator (short-circuit).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOp {
    And,
    Or,
    NullishCoalescing,
}

/// Compound assignment operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignOp {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    PowAssign,
    ShlAssign,
    ShrAssign,
    UShrAssign,
    AndAssign,
    OrAssign,
    XorAssign,
}

/// Member access kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemberOp {
    Dot,
    Bracket,
}

/// An object property in a literal.
#[derive(Debug, Clone)]
pub struct Property {
    pub key: PropertyKey,
    pub value: Expr,
    pub kind: PropertyKind,
}

/// Property key: identifier name, string, or computed (`[expr]`).
#[derive(Debug, Clone)]
pub enum PropertyKey {
    Ident(String),
    String(String),
    Number(f64),
    Computed(Box<Expr>),
}

/// Property kind: normal init, get accessor, set accessor, or method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyKind {
    Init,
    Get,
    Set,
    Method,
}

/// A function parameter.
#[derive(Debug, Clone)]
pub struct Param {
    pub pattern: BindingPattern,
    pub default: Option<Expr>,
}

/// A binding pattern (destructuring).
#[derive(Debug, Clone)]
pub enum BindingPattern {
    Identifier(String),
    Array(Vec<Option<BindingPattern>>),
    Object(Vec<ObjectBindingProperty>),
    Rest(Box<BindingPattern>),
}

/// An object destructuring property.
#[derive(Debug, Clone)]
pub struct ObjectBindingProperty {
    pub key: PropertyKey,
    pub pattern: BindingPattern,
    pub default: Option<Expr>,
}

/// Arrow function body — either a single expression or a block.
#[derive(Debug, Clone)]
pub enum ArrowBody {
    Expr(Box<Expr>),
    Block(Vec<Stmt>),
}

/// A class method.
#[derive(Debug, Clone)]
pub struct ClassMethod {
    pub key: PropertyKey,
    pub params: Vec<Param>,
    pub body: Vec<Stmt>,
    pub kind: ClassMethodKind,
    pub is_static: bool,
    pub is_async: bool,
    pub is_generator: bool,
}

/// Class method kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassMethodKind {
    Method,
    Constructor,
    Get,
    Set,
}
