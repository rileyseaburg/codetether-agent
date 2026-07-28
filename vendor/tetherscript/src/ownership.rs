//! Lightweight static ownership analysis.
//!
//! This pass catches the statically obvious ownership violations before runtime:
//! use-after-move, moving while borrowed, and shared-vs-mutable borrow conflicts
//! for simple identifier borrows bound in lexical scopes.

use std::collections::{HashMap, HashSet};

use crate::ast::{BinOp, Block, Expr, Program, Stmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub message: String,
}

impl Diagnostic {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

#[derive(Default, Clone)]
struct BindingState {
    moved: bool,
    copy: bool,
    shared_borrows: usize,
    mutable_borrow: bool,
}

#[derive(Default)]
struct Scope {
    names: HashSet<String>,
    borrows: Vec<BorrowRecord>,
}

struct BorrowRecord {
    owner: String,
    mutable: bool,
}

#[derive(Default)]
pub struct Analyzer {
    bindings: HashMap<String, BindingState>,
    scopes: Vec<Scope>,
    diagnostics: Vec<Diagnostic>,
}

impl Analyzer {
    pub fn analyze(program: &Program) -> Result<(), Vec<Diagnostic>> {
        let mut analyzer = Self::default();
        analyzer.push_scope();
        for stmt in &program.stmts {
            analyzer.stmt(stmt);
        }
        analyzer.pop_scope();
        if analyzer.diagnostics.is_empty() {
            Ok(())
        } else {
            Err(analyzer.diagnostics)
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    fn pop_scope(&mut self) {
        if let Some(scope) = self.scopes.pop() {
            for borrow in scope.borrows.into_iter().rev() {
                if let Some(owner) = self.bindings.get_mut(&borrow.owner) {
                    if borrow.mutable {
                        owner.mutable_borrow = false;
                    } else if owner.shared_borrows > 0 {
                        owner.shared_borrows -= 1;
                    }
                }
            }
            for name in scope.names {
                self.bindings.remove(&name);
            }
        }
    }

    fn define(&mut self, name: &str, copy: bool) {
        self.bindings.insert(
            name.to_string(),
            BindingState {
                copy,
                ..BindingState::default()
            },
        );
        if let Some(scope) = self.scopes.last_mut() {
            scope.names.insert(name.to_string());
        }
    }

    fn stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, value, .. } => {
                let copy = self.expr_is_copy(value);
                self.expr(value);
                self.define(name, copy);
                if let Some((owner, mutable)) = borrowed_ident(value) {
                    self.register_borrow(name, owner, mutable);
                }
            }
            Stmt::Expr { expr, .. } => self.expr(expr),
            Stmt::FnDecl { name, body, .. } => {
                self.define(name, false);
                self.block(body);
            }
        }
    }

    fn block(&mut self, block: &Block) {
        self.push_scope();
        for stmt in &block.stmts {
            self.stmt(stmt);
        }
        self.pop_scope();
    }

    fn expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Ident(name) => self.use_binding(name),
            Expr::Move(inner) => self.move_expr(inner),
            Expr::Borrow(inner) => {
                if let Expr::Ident(name) = inner.as_ref() {
                    self.check_borrow(name, false);
                }
                self.expr(inner);
            }
            Expr::BorrowMut(inner) => {
                if let Expr::Ident(name) = inner.as_ref() {
                    self.check_borrow(name, true);
                }
                self.expr(inner);
            }
            Expr::Binary { op, lhs, rhs } => {
                if *op == BinOp::Assign {
                    let copy = self.expr_is_copy(rhs);
                    self.expr(rhs);
                    if let Expr::Ident(name) = lhs.as_ref() {
                        self.assign_binding(name, copy);
                    } else {
                        self.expr(lhs);
                    }
                } else {
                    self.expr(lhs);
                    self.expr(rhs);
                }
            }
            Expr::Unary { rhs, .. }
            | Expr::Try(rhs)
            | Expr::Panic(rhs)
            | Expr::Await(rhs)
            | Expr::Spawn(rhs) => self.expr(rhs),
            Expr::AsyncFn { body, .. } => self.block(body),
            Expr::Join(exprs) => {
                for expr in exprs {
                    self.expr(expr);
                }
            }
            Expr::Call { callee, args } => {
                self.expr(callee);
                for arg in args {
                    self.expr(arg);
                }
            }
            Expr::Index { target, index } => {
                self.expr(target);
                self.expr(index);
            }
            Expr::Field { target, .. } => self.expr(target),
            Expr::Method { target, args, .. } => {
                self.expr(target);
                for arg in args {
                    self.expr(arg);
                }
            }
            Expr::List(items) => {
                for item in items {
                    self.expr(item);
                }
            }
            Expr::If {
                cond,
                then_branch,
                else_branch,
            } => {
                self.expr(cond);
                self.block(then_branch);
                if let Some(block) = else_branch {
                    self.block(block);
                }
            }
            Expr::While { cond, body } => {
                self.expr(cond);
                self.block(body);
            }
            Expr::For { iter, body, name } => {
                self.expr(iter);
                self.push_scope();
                self.define(name, false);
                for stmt in &body.stmts {
                    self.stmt(stmt);
                }
                self.pop_scope();
            }
            Expr::Block(block) => self.block(block),
            Expr::Fn { body, .. } => self.block(body),
            Expr::Return(Some(value)) => self.expr(value),
            Expr::Return(None)
            | Expr::Int(_)
            | Expr::Float(_)
            | Expr::Str(_)
            | Expr::Bytes(_)
            | Expr::Bool(_)
            | Expr::Nil => {}
        }
    }

    fn use_binding(&mut self, name: &str) {
        if let Some(state) = self.bindings.get(name) {
            if state.moved {
                self.diagnostics
                    .push(Diagnostic::new(format!("use of moved value `{}`", name)));
            }
        }
    }

    fn move_expr(&mut self, inner: &Expr) {
        if let Expr::Ident(name) = inner {
            self.move_binding(name);
        } else {
            self.expr(inner);
        }
    }

    fn move_binding(&mut self, name: &str) {
        match self.bindings.get_mut(name) {
            Some(state) => {
                if state.moved {
                    self.diagnostics.push(Diagnostic::new(format!(
                        "cannot move `{}` because it was already moved",
                        name
                    )));
                }
                if state.shared_borrows > 0 || state.mutable_borrow {
                    self.diagnostics.push(Diagnostic::new(format!(
                        "cannot move `{}` while it is borrowed",
                        name
                    )));
                }
                if !state.copy {
                    state.moved = true;
                }
            }
            None => self.diagnostics.push(Diagnostic::new(format!(
                "cannot move undefined binding `{}`",
                name
            ))),
        }
    }

    fn check_borrow(&mut self, name: &str, mutable: bool) {
        match self.bindings.get(name) {
            Some(state) if state.moved => self.diagnostics.push(Diagnostic::new(format!(
                "cannot borrow moved value `{}`",
                name
            ))),
            Some(state) if mutable && (state.shared_borrows > 0 || state.mutable_borrow) => {
                self.diagnostics.push(Diagnostic::new(format!(
                    "cannot mutably borrow `{}` while it is already borrowed",
                    name
                )));
            }
            Some(state) if !mutable && state.mutable_borrow => {
                self.diagnostics.push(Diagnostic::new(format!(
                    "cannot borrow `{}` while it is mutably borrowed",
                    name
                )));
            }
            Some(_) => {}
            None => self.diagnostics.push(Diagnostic::new(format!(
                "cannot borrow undefined binding `{}`",
                name
            ))),
        }
    }

    fn register_borrow(&mut self, borrower: &str, owner: &str, mutable: bool) {
        if let Some(state) = self.bindings.get_mut(owner) {
            if mutable {
                state.mutable_borrow = true;
            } else {
                state.shared_borrows += 1;
            }
        }
        if let Some(scope) = self.scopes.last_mut() {
            scope.borrows.push(BorrowRecord {
                owner: owner.to_string(),
                mutable,
            });
        }
        let _ = borrower;
    }

    fn assign_binding(&mut self, name: &str, copy: bool) {
        if let Some(state) = self.bindings.get_mut(name) {
            if state.shared_borrows > 0 || state.mutable_borrow {
                self.diagnostics.push(Diagnostic::new(format!(
                    "cannot assign to `{}` while it is borrowed",
                    name
                )));
            }
            state.moved = false;
            state.copy = copy;
        }
    }

    fn expr_is_copy(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Int(_) | Expr::Float(_) | Expr::Bool(_) | Expr::Nil => true,
            Expr::Ident(name) => self
                .bindings
                .get(name)
                .map(|state| state.copy)
                .unwrap_or(false),
            Expr::Move(inner) => self.expr_is_copy(inner),
            Expr::Unary { op, .. } => matches!(op, crate::ast::UnOp::Neg | crate::ast::UnOp::Not),
            Expr::Binary { op, .. } => matches!(
                op,
                BinOp::Eq | BinOp::NotEq | BinOp::Lt | BinOp::Gt | BinOp::LtEq | BinOp::GtEq
            ),
            _ => false,
        }
    }
}

fn borrowed_ident(expr: &Expr) -> Option<(&str, bool)> {
    match expr {
        Expr::Borrow(inner) => match inner.as_ref() {
            Expr::Ident(name) => Some((name.as_str(), false)),
            _ => None,
        },
        Expr::BorrowMut(inner) => match inner.as_ref() {
            Expr::Ident(name) => Some((name.as_str(), true)),
            _ => None,
        },
        _ => None,
    }
}

pub fn analyze(program: &Program) -> Result<(), Vec<Diagnostic>> {
    Analyzer::analyze(program)
}
