//! AST -> bytecode compiler.
//!
//! One `Chunk` per function body; the top-level program is itself a chunk that
//! runs once at startup. Control flow (`if`, `while`, short-circuit operators)
//! is lowered to conditional jumps with back-patched offsets. Block scopes are
//! bracketed by `PushScope`/`PopScope` so nested `if`/`while`/`{}` match the
//! tree-walker's env nesting exactly.

use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::*;
use crate::bytecode::*;
use crate::value::Value;

pub struct Compiler {
    chunk: Chunk,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::default(),
        }
    }

    /// Compile a top-level program into a chunk. Top-level `fn` declarations
    /// are hoisted so forward references work (e.g. `main` calling `fib`
    /// defined later in the file).
    pub fn compile_program(program: &Program) -> Chunk {
        let mut c = Compiler::new();
        for stmt in &program.stmts {
            if let Stmt::FnDecl { name, params, body } = stmt {
                c.compile_fn_decl(name, params, body);
            }
        }
        for stmt in &program.stmts {
            if matches!(stmt, Stmt::FnDecl { .. }) {
                continue;
            }
            c.compile_stmt(stmt);
        }
        c.emit(Instr::Nil);
        c.emit(Instr::Return);
        c.chunk
    }

    fn add_const(&mut self, v: Value) -> u16 {
        // Dedup: reuse existing constant if identical.
        if let Some(idx) = self.chunk.consts.iter().position(|c| values_equal(c, &v)) {
            return idx as u16;
        }
        let idx = self.chunk.consts.len();
        self.chunk.consts.push(v);
        idx as u16
    }

    fn intern_name(&mut self, name: &str) -> u16 {
        if let Some(i) = self.chunk.names.iter().position(|n| n == name) {
            return i as u16;
        }
        let idx = self.chunk.names.len();
        self.chunk.names.push(name.to_string());
        idx as u16
    }

    fn emit(&mut self, i: Instr) -> usize {
        let pos = self.chunk.code.len();
        self.chunk.code.push(i);
        pos
    }

    /// Back-patch the jump at `pos` so it lands at the current end of code.
    fn patch_jump(&mut self, pos: usize) {
        let offset = self.chunk.code.len() as i32 - (pos + 1) as i32;
        self.chunk.code[pos] = match &self.chunk.code[pos] {
            Instr::Jump(_) => Instr::Jump(offset),
            Instr::JumpIfFalse(_) => Instr::JumpIfFalse(offset),
            Instr::JumpIfFalseKeep(_) => Instr::JumpIfFalseKeep(offset),
            Instr::JumpIfTrueKeep(_) => Instr::JumpIfTrueKeep(offset),
            Instr::ForNext(name, _) => Instr::ForNext(*name, offset),
            other => panic!("patch_jump on non-jump instruction: {:?}", other),
        };
    }

    fn compile_fn_decl(&mut self, name: &str, params: &[String], body: &Rc<Block>) {
        let proto = Self::compile_fn(Some(name.to_string()), params, body);
        let proto_idx = self.chunk.protos.len() as u16;
        self.chunk.protos.push(Rc::new(proto));
        self.emit(Instr::MakeFn(proto_idx));
        let name_idx = self.intern_name(name);
        self.emit(Instr::DefLet(name_idx, false));
    }

    fn compile_fn(name: Option<String>, params: &[String], body: &Rc<Block>) -> FnProto {
        let mut inner = Compiler::new();
        inner.compile_block(body);
        inner.emit(Instr::Return);

        FnProto {
            name,
            params: params.to_vec(),
            chunk: inner.chunk,
        }
    }

    fn compile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let {
                name,
                mutable,
                value,
            } => {
                self.compile_expr(value);
                let idx = self.intern_name(name);
                self.emit(Instr::DefLet(idx, *mutable));
            }
            Stmt::Expr { expr, .. } => {
                self.compile_expr(expr);
                self.emit(Instr::Pop);
            }
            Stmt::FnDecl { name, params, body } => {
                self.compile_fn_decl(name, params, body);
            }
        }
    }

    /// Compile a block as an expression: leaves exactly one value on the
    /// stack — the trailing non-terminated expression, or `Nil`.
    fn compile_block(&mut self, block: &Block) {
        if block.stmts.is_empty() {
            self.emit(Instr::Nil);
            return;
        }
        for (i, stmt) in block.stmts.iter().enumerate() {
            let is_last = i == block.stmts.len() - 1;
            match stmt {
                Stmt::Expr { expr, terminated } => {
                    self.compile_expr(expr);
                    if is_last && !terminated {
                        // value stays on stack as the block's value
                    } else {
                        self.emit(Instr::Pop);
                        if is_last {
                            self.emit(Instr::Nil);
                        }
                    }
                }
                Stmt::Let {
                    name,
                    mutable,
                    value,
                } => {
                    self.compile_expr(value);
                    let idx = self.intern_name(name);
                    self.emit(Instr::DefLet(idx, *mutable));

                    if is_last {
                        self.emit(Instr::Nil);
                    }
                }
                Stmt::FnDecl { name, params, body } => {
                    self.compile_fn_decl(name, params, body);
                    if is_last {
                        self.emit(Instr::Nil);
                    }
                }
            }
        }
    }

    fn compile_expr(&mut self, e: &Expr) {
        match e {
            Expr::Int(n) => {
                let c = self.add_const(Value::Int(*n));
                self.emit(Instr::Const(c));
            }
            Expr::Float(n) => {
                let c = self.add_const(Value::Float(*n));
                self.emit(Instr::Const(c));
            }
            Expr::Str(s) => {
                let c = self.add_const(Value::Str(Rc::new(s.clone())));
                self.emit(Instr::Const(c));
            }
            Expr::Bytes(bytes) => {
                let c = self.add_const(Value::Bytes(Rc::new(RefCell::new(bytes.clone()))));
                self.emit(Instr::Const(c));
            }
            Expr::Bool(b) => {
                self.emit(if *b { Instr::True } else { Instr::False });
            }
            Expr::Nil => {
                self.emit(Instr::Nil);
            }

            Expr::Ident(name) => {
                let idx = self.intern_name(name);
                self.emit(Instr::GetName(idx));
            }

            Expr::Move(inner) => match inner.as_ref() {
                Expr::Ident(name) => {
                    let idx = self.intern_name(name);
                    self.emit(Instr::GetMove(idx));
                }
                other => self.compile_expr(other),
            },

            // Borrow operators are still implicit at runtime; emit the inner
            // expression verbatim. When `&mut` exclusivity lands, this is the
            // hook.
            Expr::Borrow(inner) | Expr::BorrowMut(inner) => self.compile_expr(inner),

            Expr::Unary { op, rhs } => {
                self.compile_expr(rhs);
                self.emit(match op {
                    UnOp::Neg => Instr::Neg,
                    UnOp::Not => Instr::Not,
                });
            }

            Expr::Binary { op, lhs, rhs } => {
                if *op == BinOp::Assign {
                    self.compile_assign(lhs, rhs);
                    return;
                }
                if *op == BinOp::And {
                    self.compile_expr(lhs);
                    let j = self.emit(Instr::JumpIfFalseKeep(0));
                    self.emit(Instr::Pop);
                    self.compile_expr(rhs);
                    self.patch_jump(j);
                    return;
                }
                if *op == BinOp::Or {
                    self.compile_expr(lhs);
                    let j = self.emit(Instr::JumpIfTrueKeep(0));
                    self.emit(Instr::Pop);
                    self.compile_expr(rhs);
                    self.patch_jump(j);
                    return;
                }
                self.compile_expr(lhs);
                self.compile_expr(rhs);
                self.emit(match op {
                    BinOp::Add => Instr::Add,
                    BinOp::Sub => Instr::Sub,
                    BinOp::Mul => Instr::Mul,
                    BinOp::Div => Instr::Div,
                    BinOp::Mod => Instr::Mod,
                    BinOp::Eq => Instr::Eq,
                    BinOp::NotEq => Instr::NotEq,
                    BinOp::Lt => Instr::Lt,
                    BinOp::Gt => Instr::Gt,
                    BinOp::LtEq => Instr::LtEq,
                    BinOp::GtEq => Instr::GtEq,
                    BinOp::And | BinOp::Or | BinOp::Assign => unreachable!(),
                });
            }

            Expr::List(items) => {
                for it in items {
                    self.compile_expr(it);
                }
                self.emit(Instr::BuildList(items.len() as u16));
            }

            Expr::Call { callee, args } => {
                self.compile_expr(callee);
                for a in args {
                    self.compile_expr(a);
                }
                self.emit(Instr::Call(args.len() as u8));
            }

            Expr::Index { target, index } => {
                self.compile_expr(target);
                self.compile_expr(index);
                self.emit(Instr::Index);
            }

            Expr::Field { target, name } => {
                self.compile_expr(target);
                let idx = self.intern_name(name);
                self.emit(Instr::GetField(idx));
            }

            Expr::Method { target, name, args } => {
                self.compile_expr(target);
                for a in args {
                    self.compile_expr(a);
                }
                let idx = self.intern_name(name);
                self.emit(Instr::Method(idx, args.len() as u8));
            }

            Expr::If {
                cond,
                then_branch,
                else_branch,
            } => {
                self.compile_expr(cond);
                let jf = self.emit(Instr::JumpIfFalse(0));
                self.emit(Instr::PushScope);
                self.compile_block(then_branch);
                self.emit(Instr::PopScope);
                let je = self.emit(Instr::Jump(0));
                self.patch_jump(jf);
                if let Some(eb) = else_branch {
                    self.emit(Instr::PushScope);
                    self.compile_block(eb);
                    self.emit(Instr::PopScope);
                } else {
                    self.emit(Instr::Nil);
                }
                self.patch_jump(je);
            }

            Expr::While { cond, body } => {
                let loop_start = self.chunk.code.len();
                self.compile_expr(cond);
                let jf = self.emit(Instr::JumpIfFalse(0));
                self.emit(Instr::PushScope);
                self.compile_block(body);
                self.emit(Instr::Pop);
                self.emit(Instr::PopScope);
                let back = loop_start as i32 - (self.chunk.code.len() + 1) as i32;
                self.emit(Instr::Jump(back));
                self.patch_jump(jf);
                self.emit(Instr::Nil);
            }

            Expr::For { name, iter, body } => {
                self.compile_expr(iter);
                self.emit(Instr::IterInit);
                let zero = self.add_const(Value::Int(0));
                self.emit(Instr::Const(zero));
                self.emit(Instr::PushScope);

                // Keep loop variables environment-backed. `ForNext` also keeps
                // the next index on the stack, so treating that value as a
                // local binding would corrupt the loop state.
                let name_idx = self.intern_name(name);
                let loop_start = self.chunk.code.len();
                let exhausted = self.emit(Instr::ForNext(name_idx, 0));

                self.emit(Instr::PushScope);
                self.compile_block(body);
                self.emit(Instr::Pop);
                self.emit(Instr::PopScope);
                let back = loop_start as i32 - (self.chunk.code.len() + 1) as i32;
                self.emit(Instr::Jump(back));
                self.patch_jump(exhausted);
                self.emit(Instr::PopScope);
                self.emit(Instr::Nil);
            }

            Expr::Block(block) => {
                self.emit(Instr::PushScope);
                self.compile_block(block);
                self.emit(Instr::PopScope);
            }

            Expr::Fn { params, body } => {
                let proto = Self::compile_fn(None, params, body);
                let idx = self.chunk.protos.len() as u16;
                self.chunk.protos.push(Rc::new(proto));
                self.emit(Instr::MakeFn(idx));
            }

            Expr::Return(inner) => {
                match inner {
                    Some(e) => self.compile_expr(e),
                    None => {
                        self.emit(Instr::Nil);
                    }
                }
                self.emit(Instr::Return);
            }

            Expr::Panic(msg) => {
                self.compile_expr(msg);
                self.emit(Instr::Panic);
            }

            Expr::Try(inner) => {
                self.compile_expr(inner);
                self.emit(Instr::Try);
            }

            Expr::AsyncFn { params, body } => {
                let proto = Self::compile_fn(None, params, body);
                let idx = self.chunk.protos.len() as u16;
                self.chunk.protos.push(Rc::new(proto));
                self.emit(Instr::MakeFn(idx));
            }
            Expr::Await(inner) | Expr::Spawn(inner) => {
                self.compile_expr(inner);
            }
            Expr::Join(handles) => {
                for h in handles {
                    self.compile_expr(h);
                }
                self.emit(Instr::BuildList(handles.len() as u16));
            }
        }
    }

    fn compile_assign(&mut self, lhs: &Expr, rhs: &Expr) {
        match lhs {
            Expr::Ident(name) => {
                self.compile_expr(rhs);
                let idx = self.intern_name(name);
                self.emit(Instr::Assign(idx));
            }
            Expr::Index { target, index } => {
                self.compile_expr(target);
                self.compile_expr(index);
                self.compile_expr(rhs);
                self.emit(Instr::SetIndex);
            }
            Expr::Field { target, name } => {
                self.compile_expr(target);
                self.compile_expr(rhs);
                let idx = self.intern_name(name);
                self.emit(Instr::SetField(idx));
            }
            _ => {
                let c = self.add_const(Value::Str(Rc::new("invalid assignment target".into())));
                self.emit(Instr::Const(c));
                self.emit(Instr::Panic);
            }
        }
    }
}

/// Compare two Values for constant pool deduplication.
fn values_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => x == y,
        (Value::Float(x), Value::Float(y)) => x.to_bits() == y.to_bits(),
        (Value::Bool(x), Value::Bool(y)) => x == y,
        (Value::Nil, Value::Nil) => true,
        (Value::Str(x), Value::Str(y)) => Rc::ptr_eq(x, y) || **x == **y,
        _ => false,
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
