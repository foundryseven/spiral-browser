//! Statement AST nodes.

use super::expr::Expr;
use super::Span;

/// A statement node.
#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

/// The discriminant for statement variants.
#[derive(Debug, Clone)]
pub enum StmtKind {
    /// Expression statement: `expr;`.
    Expr(Expr),
    /// `var x = 1;`
    Var { declarations: Vec<VarDeclarator> },
    /// `let x = 1;`
    Let { declarations: Vec<VarDeclarator> },
    /// `const x = 1;`
    Const { declarations: Vec<VarDeclarator> },
    /// `if (cond) { ... } else { ... }`
    If {
        test: Expr,
        consequent: Box<Stmt>,
        alternate: Option<Box<Stmt>>,
    },
    /// `for (init; test; update) { ... }`
    For {
        init: Option<ForInit>,
        test: Option<Expr>,
        update: Option<Expr>,
        body: Box<Stmt>,
    },
    /// `for (var/let/const x of/iter) { ... }`
    ForOf {
        left: ForInit,
        right: Expr,
        body: Box<Stmt>,
        is_await: bool,
    },
    /// `for (var/let/const x in obj) { ... }`
    ForIn {
        left: ForInit,
        right: Expr,
        body: Box<Stmt>,
    },
    /// `while (cond) { ... }`
    While { test: Expr, body: Box<Stmt> },
    /// `do { ... } while (cond);`
    DoWhile { body: Box<Stmt>, test: Expr },
    /// `switch (expr) { case ... }`
    Switch {
        discriminant: Expr,
        cases: Vec<SwitchCase>,
    },
    /// `break label;`
    Break { label: Option<String> },
    /// `continue label;`
    Continue { label: Option<String> },
    /// `return expr;`
    Return { value: Option<Expr> },
    /// `throw expr;`
    Throw { value: Expr },
    /// `try { ... } catch (e) { ... } finally { ... }`
    Try {
        block: Vec<Stmt>,
        handler: Option<CatchClause>,
        finalizer: Option<Vec<Stmt>>,
    },
    /// `{ ... }` block.
    Block(Vec<Stmt>),
    /// `label: stmt`
    Labelled { label: String, body: Box<Stmt> },
    /// `function name(params) { ... }` (hoisted declaration).
    FunctionDecl {
        name: String,
        params: Vec<super::expr::Param>,
        body: Vec<Stmt>,
        is_async: bool,
        is_generator: bool,
    },
    /// `class Name { ... }` (hoisted declaration).
    ClassDecl {
        name: String,
        super_class: Option<Expr>,
        methods: Vec<super::expr::ClassMethod>,
    },
    /// `import ... from "...";`
    Import {
        specifiers: Vec<ImportSpecifier>,
        source: String,
    },
    /// `export default expr;` / `export { ... }` / `export function ...`
    Export { declaration: ExportDeclaration },
    /// Empty statement: `;`
    Empty,
    /// `"use strict";` (expression statement that triggers directive mode).
    Directive { value: String },
    /// `debugger;`
    Debugger,
}

/// A variable declarator: `x = expr`.
#[derive(Debug, Clone)]
pub struct VarDeclarator {
    pub pattern: super::expr::BindingPattern,
    pub init: Option<Expr>,
}

/// The left-hand side of a `for` loop.
#[derive(Debug, Clone)]
pub enum ForInit {
    Var(Vec<VarDeclarator>),
    Let(Vec<VarDeclarator>),
    Const(Vec<VarDeclarator>),
    Expr(Expr),
}

/// A `case` or `default` clause in a `switch` statement.
#[derive(Debug, Clone)]
pub struct SwitchCase {
    pub test: Option<Expr>,
    pub consequent: Vec<Stmt>,
}

/// A `catch` clause.
#[derive(Debug, Clone)]
pub struct CatchClause {
    pub param: Option<super::expr::BindingPattern>,
    pub body: Vec<Stmt>,
}

/// An import specifier.
#[derive(Debug, Clone)]
pub enum ImportSpecifier {
    /// `import defaultExport from "..."`
    Default { local: String },
    /// `import { name } from "..."`
    Named { imported: String, local: String },
    /// `import * as name from "..."`
    Namespace { local: String },
}

/// An export declaration.
#[derive(Debug, Clone)]
pub enum ExportDeclaration {
    /// `export default expr`
    Default(Expr),
    /// `export { a, b as c }`
    Named(Vec<ExportSpecifier>),
    /// `export function name() { ... }` / `export class Name { ... }` /
    /// `export const x = 1`
    Decl(Box<StmtKind>),
    /// `export * from "..."`
    AllFrom(String),
}

/// An export specifier.
#[derive(Debug, Clone)]
pub struct ExportSpecifier {
    pub local: String,
    pub exported: String,
}
