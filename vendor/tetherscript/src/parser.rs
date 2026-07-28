//! Parser — tokens → AST.
//!
//! Pratt parser for expressions, recursive descent for statements.
//! Newlines are mostly treated as whitespace here, but we use them for
//! automatic semicolon insertion at the end of a statement.

use std::rc::Rc;

use crate::ast::*;
use crate::token::{Spanned, Token};

pub struct Parser {
    tokens: Vec<Spanned>,
    pos: usize,
}

#[derive(Debug)]
pub struct ParseError {
    pub msg: String,
    pub line: usize,
    pub col: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Spanned>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut stmts = Vec::new();
        self.skip_newlines();
        while !self.at_end() {
            stmts.push(self.parse_stmt()?);
            self.skip_newlines();
        }
        Ok(Program { stmts })
    }

    // ---------- helpers ----------

    fn peek(&self) -> &Token {
        &self.tokens[self.pos].token
    }

    fn peek_spanned(&self) -> &Spanned {
        &self.tokens[self.pos]
    }

    fn at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }

    fn bump(&mut self) -> Spanned {
        let t = self.tokens[self.pos].clone();
        if !self.at_end() {
            self.pos += 1;
        }
        t
    }

    fn check(&self, t: &Token) -> bool {
        std::mem::discriminant(self.peek()) == std::mem::discriminant(t)
    }

    fn eat(&mut self, t: &Token) -> bool {
        if self.check(t) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, t: &Token, what: &str) -> Result<Spanned, ParseError> {
        if self.check(t) {
            Ok(self.bump())
        } else {
            Err(self.error(format!("expected {}, got {:?}", what, self.peek())))
        }
    }

    fn skip_newlines(&mut self) {
        while matches!(self.peek(), Token::Newline) {
            self.bump();
        }
    }

    fn error(&self, msg: String) -> ParseError {
        let s = self.peek_spanned();
        ParseError {
            msg,
            line: s.line,
            col: s.col,
        }
    }

    // ---------- statements ----------

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        match self.peek() {
            Token::Let => self.parse_let(),
            Token::Async
                if matches!(
                    self.tokens.get(self.pos + 1).map(|s| &s.token),
                    Some(Token::Fn)
                ) && matches!(
                    self.tokens.get(self.pos + 2).map(|s| &s.token),
                    Some(Token::Ident(_))
                ) =>
            {
                self.bump();
                self.parse_fn_decl()
            }
            // `fn` followed by an identifier is a declaration; `fn (` is an
            // anonymous function expression (e.g. returned from a block).
            Token::Fn
                if matches!(
                    self.tokens.get(self.pos + 1).map(|s| &s.token),
                    Some(Token::Ident(_))
                ) =>
            {
                self.parse_fn_decl()
            }
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_let(&mut self) -> Result<Stmt, ParseError> {
        self.bump(); // `let`
        let mutable = self.eat(&Token::Mut);
        let name = match self.bump().token {
            Token::Ident(s) => s,
            other => return Err(self.error(format!("expected identifier, got {:?}", other))),
        };
        self.expect(&Token::Assign, "`=`")?;
        let value = self.parse_expr()?;
        self.consume_stmt_end();
        Ok(Stmt::Let {
            name,
            mutable,
            value,
        })
    }

    fn parse_fn_decl(&mut self) -> Result<Stmt, ParseError> {
        self.bump(); // `fn`
        let name = match self.bump().token {
            Token::Ident(s) => s,
            other => return Err(self.error(format!("expected fn name, got {:?}", other))),
        };
        let params = self.parse_params()?;
        let body = Rc::new(self.parse_block()?);
        Ok(Stmt::FnDecl { name, params, body })
    }

    fn parse_params(&mut self) -> Result<Vec<String>, ParseError> {
        self.expect(&Token::LParen, "`(`")?;
        let mut params = Vec::new();
        self.skip_newlines();
        while !self.check(&Token::RParen) {
            let name = match self.bump().token {
                Token::Ident(s) => s,
                other => {
                    return Err(self.error(format!("expected parameter name, got {:?}", other)))
                }
            };
            params.push(name);
            self.skip_newlines();
            if !self.eat(&Token::Comma) {
                break;
            }
            self.skip_newlines();
        }
        self.expect(&Token::RParen, "`)`")?;
        Ok(params)
    }

    fn parse_expr_stmt(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.parse_expr()?;
        let terminated = self.consume_stmt_end();
        Ok(Stmt::Expr { expr, terminated })
    }

    /// Consume an end-of-statement: either `;`, newline(s), or EOF/`}`.
    /// Returns true if a `;` was consumed (meaning the block should discard
    /// this expression's value).
    fn consume_stmt_end(&mut self) -> bool {
        let had_semi = self.eat(&Token::Semi);
        self.skip_newlines();
        had_semi
    }

    // ---------- blocks ----------

    fn parse_block(&mut self) -> Result<Block, ParseError> {
        self.expect(&Token::LBrace, "`{`")?;
        self.skip_newlines();
        let mut stmts = Vec::new();
        while !self.check(&Token::RBrace) && !self.at_end() {
            stmts.push(self.parse_stmt()?);
            self.skip_newlines();
        }
        self.expect(&Token::RBrace, "`}`")?;
        Ok(Block { stmts })
    }

    // ---------- expressions (Pratt) ----------

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_precedence(Prec::Assign)
    }

    fn parse_precedence(&mut self, min: Prec) -> Result<Expr, ParseError> {
        let mut lhs = self.parse_prefix()?;

        loop {
            let op_prec = infix_prec(self.peek());
            if op_prec < min {
                break;
            }

            lhs = self.parse_infix(lhs, op_prec)?;
        }

        Ok(lhs)
    }

    fn parse_prefix(&mut self) -> Result<Expr, ParseError> {
        match self.peek().clone() {
            Token::Int(n) => {
                self.bump();
                Ok(Expr::Int(n))
            }
            Token::Float(f) => {
                self.bump();
                Ok(Expr::Float(f))
            }
            Token::Str(s) => {
                self.bump();
                Ok(Expr::Str(s))
            }
            Token::Bytes(bytes) => {
                self.bump();
                Ok(Expr::Bytes(bytes))
            }
            Token::Bool(b) => {
                self.bump();
                Ok(Expr::Bool(b))
            }
            Token::Nil => {
                self.bump();
                Ok(Expr::Nil)
            }
            Token::Ident(s) => {
                self.bump();
                Ok(Expr::Ident(s))
            }

            Token::Minus => {
                self.bump();
                let rhs = self.parse_precedence(Prec::Unary)?;
                Ok(Expr::Unary {
                    op: UnOp::Neg,
                    rhs: Box::new(rhs),
                })
            }
            Token::Not => {
                self.bump();
                let rhs = self.parse_precedence(Prec::Unary)?;
                Ok(Expr::Unary {
                    op: UnOp::Not,
                    rhs: Box::new(rhs),
                })
            }
            Token::Move => {
                self.bump();
                let rhs = self.parse_precedence(Prec::Unary)?;
                Ok(Expr::Move(Box::new(rhs)))
            }
            Token::Amp => {
                self.bump();
                if self.eat(&Token::Mut) {
                    let rhs = self.parse_precedence(Prec::Unary)?;
                    Ok(Expr::BorrowMut(Box::new(rhs)))
                } else {
                    let rhs = self.parse_precedence(Prec::Unary)?;
                    Ok(Expr::Borrow(Box::new(rhs)))
                }
            }

            Token::LParen => {
                self.bump();
                self.skip_newlines();
                let e = self.parse_expr()?;
                self.skip_newlines();
                self.expect(&Token::RParen, "`)`")?;
                Ok(e)
            }

            Token::LBracket => {
                self.bump();
                self.skip_newlines();
                let mut items = Vec::new();
                while !self.check(&Token::RBracket) {
                    items.push(self.parse_expr()?);
                    self.skip_newlines();
                    if !self.eat(&Token::Comma) {
                        break;
                    }
                    self.skip_newlines();
                }
                self.expect(&Token::RBracket, "`]`")?;
                Ok(Expr::List(items))
            }

            Token::LBrace => {
                let block = self.parse_block()?;
                Ok(Expr::Block(Box::new(block)))
            }

            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::For => self.parse_for(),
            Token::Fn => self.parse_fn_expr(),
            Token::Async => self.parse_async_fn(),
            Token::Await => self.parse_await(),
            Token::Spawn => self.parse_spawn(),
            Token::Join => self.parse_join(),
            Token::Return => self.parse_return(),
            Token::Panic => self.parse_panic(),

            other => Err(self.error(format!("unexpected token in expression: {:?}", other))),
        }
    }

    fn parse_infix(&mut self, lhs: Expr, prec: Prec) -> Result<Expr, ParseError> {
        let tok = self.peek().clone();

        // Call / index / field
        match tok {
            Token::LParen => {
                self.bump();
                let args = self.parse_call_args()?;
                return Ok(Expr::Call {
                    callee: Box::new(lhs),
                    args,
                });
            }
            Token::LBracket => {
                self.bump();
                self.skip_newlines();
                let idx = self.parse_expr()?;
                self.skip_newlines();
                self.expect(&Token::RBracket, "`]`")?;
                return Ok(Expr::Index {
                    target: Box::new(lhs),
                    index: Box::new(idx),
                });
            }
            Token::Question => {
                self.bump();
                return Ok(Expr::Try(Box::new(lhs)));
            }
            Token::Dot => {
                self.bump();
                let name = match self.bump().token {
                    Token::Ident(s) => s,
                    Token::Join => "join".to_string(),
                    other => {
                        return Err(self.error(format!("expected field name, got {:?}", other)))
                    }
                };
                if self.eat(&Token::LParen) {
                    let args = self.parse_call_args()?;
                    return Ok(Expr::Method {
                        target: Box::new(lhs),
                        name,
                        args,
                    });
                }
                return Ok(Expr::Field {
                    target: Box::new(lhs),
                    name,
                });
            }
            _ => {}
        }

        // Binary operators
        let op = match tok {
            Token::Plus => BinOp::Add,
            Token::Minus => BinOp::Sub,
            Token::Star => BinOp::Mul,
            Token::Slash => BinOp::Div,
            Token::Percent => BinOp::Mod,
            Token::Eq => BinOp::Eq,
            Token::NotEq => BinOp::NotEq,
            Token::Lt => BinOp::Lt,
            Token::Gt => BinOp::Gt,
            Token::LtEq => BinOp::LtEq,
            Token::GtEq => BinOp::GtEq,
            Token::And => BinOp::And,
            Token::Or => BinOp::Or,
            Token::Assign => BinOp::Assign,
            _ => return Err(self.error(format!("unexpected infix token: {:?}", tok))),
        };
        self.bump();

        // Right-associative for assignment, left-associative for everything else.
        let next_min = if op == BinOp::Assign {
            prec
        } else {
            prec.next()
        };
        self.skip_newlines();
        let rhs = self.parse_precedence(next_min)?;
        Ok(Expr::Binary {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }

    fn parse_call_args(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut args = Vec::new();
        self.skip_newlines();
        while !self.check(&Token::RParen) {
            args.push(self.parse_expr()?);
            self.skip_newlines();
            if !self.eat(&Token::Comma) {
                break;
            }
            self.skip_newlines();
        }
        self.expect(&Token::RParen, "`)`")?;
        Ok(args)
    }

    fn parse_if(&mut self) -> Result<Expr, ParseError> {
        self.bump(); // `if`
        let cond = self.parse_expr()?;
        let then_branch = Box::new(self.parse_block()?);
        let else_branch = if self.eat(&Token::Else) {
            // `else if` chain: synthesize a block wrapping the next if.
            if matches!(self.peek(), Token::If) {
                let inner = self.parse_if()?;
                Some(Box::new(Block {
                    stmts: vec![Stmt::Expr {
                        expr: inner,
                        terminated: false,
                    }],
                }))
            } else {
                Some(Box::new(self.parse_block()?))
            }
        } else {
            None
        };
        Ok(Expr::If {
            cond: Box::new(cond),
            then_branch,
            else_branch,
        })
    }

    fn parse_while(&mut self) -> Result<Expr, ParseError> {
        self.bump(); // `while`
        let cond = self.parse_expr()?;
        let body = Box::new(self.parse_block()?);
        Ok(Expr::While {
            cond: Box::new(cond),
            body,
        })
    }

    fn parse_for(&mut self) -> Result<Expr, ParseError> {
        self.bump(); // `for`
        let name = match self.bump().token {
            Token::Ident(s) => s,
            other => return Err(self.error(format!("expected loop variable, got {:?}", other))),
        };
        self.expect(&Token::In, "`in`")?;
        let iter = self.parse_expr()?;
        let body = Box::new(self.parse_block()?);
        Ok(Expr::For {
            name,
            iter: Box::new(iter),
            body,
        })
    }

    fn parse_fn_expr(&mut self) -> Result<Expr, ParseError> {
        self.bump(); // `fn`
        let params = self.parse_params()?;
        let body = Rc::new(self.parse_block()?);
        Ok(Expr::Fn { params, body })
    }

    fn parse_return(&mut self) -> Result<Expr, ParseError> {
        self.bump(); // `return`
                     // Optional expression; if the next thing terminates the stmt, bare return.
        let expr = match self.peek() {
            Token::Semi | Token::Newline | Token::RBrace | Token::Eof => None,
            _ => Some(Box::new(self.parse_expr()?)),
        };
        Ok(Expr::Return(expr))
    }

    fn parse_async_fn(&mut self) -> Result<Expr, ParseError> {
        self.bump();
        self.expect(&Token::Fn, "`fn` after `async`")?;
        let params = self.parse_params()?;
        let body = Rc::new(self.parse_block()?);
        Ok(Expr::AsyncFn { params, body })
    }

    fn parse_await(&mut self) -> Result<Expr, ParseError> {
        self.bump();
        let expr = self.parse_expr()?;
        Ok(Expr::Await(Box::new(expr)))
    }

    fn parse_spawn(&mut self) -> Result<Expr, ParseError> {
        self.bump();
        let expr = self.parse_expr()?;
        Ok(Expr::Spawn(Box::new(expr)))
    }

    fn parse_join(&mut self) -> Result<Expr, ParseError> {
        self.bump();
        self.expect(&Token::LParen, "`(` after `join`")?;
        let mut handles = Vec::new();
        self.skip_newlines();
        while !self.check(&Token::RParen) {
            handles.push(self.parse_expr()?);
            self.skip_newlines();
            if !self.eat(&Token::Comma) {
                break;
            }
            self.skip_newlines();
        }
        self.expect(&Token::RParen, "`)`")?;
        Ok(Expr::Join(handles))
    }

    fn parse_panic(&mut self) -> Result<Expr, ParseError> {
        self.bump(); // `panic`
        let msg = self.parse_expr()?;
        Ok(Expr::Panic(Box::new(msg)))
    }
}

// ---------- precedence ----------

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum Prec {
    None,
    Assign,   // =
    Or,       // ||
    And,      // &&
    Equality, // == !=
    Compare,  // < > <= >=
    Term,     // + -
    Factor,   // * / %
    Unary,    // -x, !x, move x, &x
    Call,     // f() x[] x.y
}

impl Prec {
    fn next(self) -> Prec {
        match self {
            Prec::None => Prec::Assign,
            Prec::Assign => Prec::Or,
            Prec::Or => Prec::And,
            Prec::And => Prec::Equality,
            Prec::Equality => Prec::Compare,
            Prec::Compare => Prec::Term,
            Prec::Term => Prec::Factor,
            Prec::Factor => Prec::Unary,
            Prec::Unary => Prec::Call,
            Prec::Call => Prec::Call,
        }
    }
}

fn infix_prec(t: &Token) -> Prec {
    match t {
        Token::Assign => Prec::Assign,
        Token::Or => Prec::Or,
        Token::And => Prec::And,
        Token::Eq | Token::NotEq => Prec::Equality,
        Token::Lt | Token::Gt | Token::LtEq | Token::GtEq => Prec::Compare,
        Token::Plus | Token::Minus => Prec::Term,
        Token::Star | Token::Slash | Token::Percent => Prec::Factor,
        Token::LParen | Token::LBracket | Token::Dot | Token::Question => Prec::Call,
        _ => Prec::None,
    }
}
