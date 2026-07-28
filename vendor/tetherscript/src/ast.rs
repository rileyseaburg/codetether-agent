//! Abstract syntax tree.
//!
//! TetherScript is expression-oriented: a block is an expression whose value is its
//! trailing expression (if any). So `if`, `while`, `{ ... }` are all exprs.

use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Expr {
    // Literals
    Int(i64),
    Float(f64),
    Str(String),
    Bytes(Vec<u8>),
    Bool(bool),
    Nil,

    // Variable reference
    Ident(String),

    // Binary / unary
    Binary {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Unary {
        op: UnOp,
        rhs: Box<Expr>,
    },

    // Compound
    List(Vec<Expr>),
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Index {
        target: Box<Expr>,
        index: Box<Expr>,
    },
    Field {
        target: Box<Expr>,
        name: String,
    },
    Method {
        target: Box<Expr>,
        name: String,
        args: Vec<Expr>,
    },

    // Ownership operators
    Move(Box<Expr>),      // `move x`
    Borrow(Box<Expr>),    // `&x`
    BorrowMut(Box<Expr>), // `&mut x`

    // Control flow (all expressions)
    If {
        cond: Box<Expr>,
        then_branch: Box<Block>,
        else_branch: Option<Box<Block>>,
    },
    While {
        cond: Box<Expr>,
        body: Box<Block>,
    },
    For {
        name: String,
        iter: Box<Expr>,
        body: Box<Block>,
    },
    Block(Box<Block>),

    // Function literal (anonymous)
    Fn {
        params: Vec<String>,
        body: Rc<Block>,
    },

    // `return expr` — unwinds to enclosing fn
    Return(Option<Box<Expr>>),

    // `panic "msg"` — unconditional halt
    Panic(Box<Expr>),

    // Async/concurrency expressions.
    AsyncFn {
        params: Vec<String>,
        body: Rc<Block>,
    },
    Await(Box<Expr>),
    Spawn(Box<Expr>),
    Join(Vec<Expr>),

    // `expr?` — if expr is Err(e), short-circuit out of the enclosing fn
    // by returning Err(e); if Ok(v), evaluate to v. See interp for semantics.
    Try(Box<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    And,
    Or,
    Assign,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnOp {
    Neg,
    Not,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    /// `let [mut] name = expr`
    Let {
        name: String,
        mutable: bool,
        value: Expr,
    },
    /// Bare expression statement (with or without trailing `;`).
    /// `terminated` = true means there was a semicolon, so the block should
    /// NOT use this expr as its value.
    Expr { expr: Expr, terminated: bool },
    /// `fn name(params) { body }` — sugar for `let name = fn(...) { ... }`,
    /// but we keep it as a distinct form so top-level fns can be hoisted.
    FnDecl {
        name: String,
        params: Vec<String>,
        body: Rc<Block>,
    },
}

#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}
