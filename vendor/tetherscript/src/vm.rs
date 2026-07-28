//! Stack-based bytecode VM.
//!
//! Executes chunks produced by `compiler.rs`. Values live on a Vec stack;
//! bindings live in `Env` slots (same as the tree-walker) so the move /
//! borrow / Copy / tombstone rules apply byte-for-byte identically.
//!
//! Each active call is a `Frame` holding its chunk, instruction pointer, and
//! current env. `PushScope`/`PopScope` nest envs within a frame; `Call`
//! pushes a new frame; `Return` pops one.
//!
//! Local-slot opcodes exist for a future optimization, but the compiler keeps
//! user bindings environment-backed so runtime ownership tombstones and
//! mutability checks match the reference interpreter.

use std::cell::RefCell;
use std::rc::Rc;

use crate::bytecode::{Chunk, FnProto, Instr, VmFnObj};
use crate::interp::{
    apply_binary, apply_unary, call_method, field_value, index_value, install_builtins,
    iterable_values,
};
use crate::value::{Env, NativeFunc, Runtime, Value};

pub enum Unwind {
    Error(String),
    Panic(String),
    TryErr(String),
}

impl From<String> for Unwind {
    fn from(s: String) -> Self {
        Unwind::Error(s)
    }
}

struct Frame {
    proto: Rc<FnProto>,
    ip: usize,
    env: Rc<RefCell<Env>>,
    /// Fast local variable slots, indexed by compiler-assigned slot number.
    locals: Vec<Value>,
}

pub struct VM {
    globals: Rc<RefCell<Env>>,
    stack: Vec<Value>,
    frames: Vec<Frame>,
    instruction_budget: Option<u64>,
}

impl VM {
    pub fn new() -> Self {
        Self::builder().build()
    }

    pub fn builder() -> VmBuilder {
        VmBuilder::new()
    }

    pub fn set_instruction_budget(&mut self, budget: Option<u64>) {
        self.instruction_budget = budget;
    }

    fn tick_instruction(&mut self) -> Result<(), Unwind> {
        if let Some(remaining) = self.instruction_budget.as_mut() {
            if *remaining == 0 {
                return Err(Unwind::Error(
                    "VM instruction budget exhausted (possible infinite loop)".into(),
                ));
            }
            *remaining -= 1;
        }
        Ok(())
    }

    fn new_with_budget(instruction_budget: Option<u64>) -> Self {
        let globals = Env::new_global();
        install_builtins(&globals);
        Self {
            globals,
            stack: Vec::with_capacity(256),
            frames: Vec::with_capacity(32),
            instruction_budget,
        }
    }

    /// Grant a capability at the given global name. See `Interpreter::grant`.
    pub fn grant(&mut self, name: &str, authority: Rc<dyn crate::capability::Authority>) {
        let cap = crate::capability::Capability::new_root(name, authority);
        self.globals
            .borrow_mut()
            .define(name, Value::Capability(cap), false);
    }

    pub fn run(&mut self, top_level: Chunk) -> Result<(), String> {
        let proto = Rc::new(FnProto {
            name: Some("<script>".into()),
            params: vec![],
            chunk: top_level,
        });
        let local_count = proto.chunk.local_count as usize;
        self.frames.push(Frame {
            proto,
            ip: 0,
            env: self.globals.clone(),
            locals: vec![Value::Nil; local_count],
        });
        if let Err(u) = self.execute() {
            return Err(format_unwind(u));
        }

        // Conventional entry point: if `main` exists, call it after the
        // top-level has finished populating globals.
        let has_main = self.globals.borrow().slots.contains_key("main");
        if has_main {
            let main = self.globals.borrow().get("main")?;
            if let Err(u) = self.dispatch_call(main, vec![]) {
                return Err(format_unwind(u));
            }
            if !self.frames.is_empty() {
                if let Err(u) = self.execute() {
                    return Err(format_unwind(u));
                }
            }
        }
        Ok(())
    }

    fn execute(&mut self) -> Result<(), Unwind> {
        while !self.frames.is_empty() {
            // Read one instruction from the current (top) frame — no clone.
            let f = self.frames.last().unwrap();
            let code_len = f.proto.chunk.code.len();
            if f.ip >= code_len {
                // Ran off the end without Return — synthesize one.
                self.stack.push(Value::Nil);
                self.do_return();
                continue;
            }
            self.tick_instruction()?;

            // Inline the hot-path instruction dispatch to avoid cloning.
            // We only clone instructions that need it for patching.
            let f = self.frames.last().unwrap();
            let instr_ptr = &f.proto.chunk.code[f.ip] as *const Instr;
            self.frames.last_mut().unwrap().ip += 1;

            // Safety: instr_ptr points into the Rc<FnProto> held by the frame,
            // which is alive for the duration of this step.
            let instr = unsafe { &*instr_ptr };
            match self.step_ref(instr, code_len) {
                Ok(()) => {}
                Err(Unwind::TryErr(e)) => {
                    // Lift `?`-propagated Err to the current fn's return value.
                    // If we're at the top frame, bubble up as a genuine error —
                    // there's no enclosing fn to catch it.
                    if self.frames.len() <= 1 {
                        return Err(Unwind::Error(format!("unhandled `?` error: {}", e)));
                    }
                    self.stack
                        .push(Value::Result(Rc::new(crate::value::ResultValue::Err(e))));
                    self.do_return();
                }
                Err(other) => return Err(other),
            }
        }
        Ok(())
    }

    /// Step using a reference to avoid cloning instructions on the hot path.
    fn step_ref(&mut self, instr: &Instr, _code_len: usize) -> Result<(), Unwind> {
        match instr {
            Instr::Pop => {
                self.stack.pop();
            }
            Instr::Const(idx) => {
                let v = clone_const_value(
                    &self.frames.last().unwrap().proto.chunk.consts[*idx as usize],
                );
                self.stack.push(v);
            }
            Instr::Nil => self.stack.push(Value::Nil),
            Instr::True => self.stack.push(Value::Bool(true)),
            Instr::False => self.stack.push(Value::Bool(false)),
            Instr::GetName(idx) => {
                let name = self.name(*idx);
                let env = self.frames.last().unwrap().env.clone();
                let v = env.borrow().get(&name)?;
                self.stack.push(v);
            }
            Instr::GetMove(idx) => {
                let name = self.name(*idx);
                let env = self.frames.last().unwrap().env.clone();
                let v = env.borrow_mut().take(&name)?;
                self.stack.push(v);
            }
            Instr::DefLet(idx, mutable) => {
                let name = self.name(*idx);
                let v = self.stack.pop().expect("DefLet with empty stack");
                self.frames
                    .last()
                    .unwrap()
                    .env
                    .borrow_mut()
                    .define(&name, v, *mutable);
            }
            Instr::Assign(idx) => {
                let name = self.name(*idx);
                let v = self.stack.pop().expect("Assign with empty stack");
                self.frames
                    .last()
                    .unwrap()
                    .env
                    .borrow_mut()
                    .assign(&name, v.clone())?;
                self.stack.push(v);
            }
            Instr::GetLocal(idx) => {
                let v = self.get_local(*idx as usize);
                self.stack.push(v);
            }
            Instr::SetLocal(idx) => {
                let v = self.stack.pop().expect("SetLocal with empty stack");
                self.set_local(*idx as usize, v.clone());
                self.stack.push(v);
            }
            Instr::DefLocal(idx, _mutable) => {
                let v = self.stack.pop().expect("DefLocal with empty stack");
                self.set_local(*idx as usize, v);
            }
            Instr::MoveLocal(idx) => {
                let v = self.take_local(*idx as usize);
                self.stack.push(v);
            }
            Instr::Neg => {
                let v = self.stack.pop().unwrap();
                self.stack.push(apply_unary(crate::ast::UnOp::Neg, v)?);
            }
            Instr::Not => {
                let v = self.stack.pop().unwrap();
                self.stack.push(apply_unary(crate::ast::UnOp::Not, v)?);
            }
            Instr::Add => self.binary(crate::ast::BinOp::Add)?,
            Instr::Sub => self.binary(crate::ast::BinOp::Sub)?,
            Instr::Mul => self.binary(crate::ast::BinOp::Mul)?,
            Instr::Div => self.binary(crate::ast::BinOp::Div)?,
            Instr::Mod => self.binary(crate::ast::BinOp::Mod)?,
            Instr::Eq => self.binary(crate::ast::BinOp::Eq)?,
            Instr::NotEq => self.binary(crate::ast::BinOp::NotEq)?,
            Instr::Lt => self.binary(crate::ast::BinOp::Lt)?,
            Instr::Gt => self.binary(crate::ast::BinOp::Gt)?,
            Instr::LtEq => self.binary(crate::ast::BinOp::LtEq)?,
            Instr::GtEq => self.binary(crate::ast::BinOp::GtEq)?,
            Instr::Jump(off) => self.jump(*off),
            Instr::JumpIfFalse(off) => {
                let v = self.stack.pop().unwrap();
                if !v.truthy() {
                    self.jump(*off);
                }
            }
            Instr::JumpIfFalseKeep(off) => {
                if !self.stack.last().unwrap().truthy() {
                    self.jump(*off);
                }
            }
            Instr::JumpIfTrueKeep(off) => {
                if self.stack.last().unwrap().truthy() {
                    self.jump(*off);
                }
            }
            Instr::IterInit => {
                let iterable = self.stack.pop().expect("IterInit with empty stack");
                let items = iterable_values(&iterable)?;
                self.stack.push(Value::List(Rc::new(RefCell::new(items))));
            }
            Instr::ForNext(idx, off) => {
                let name = self.name(*idx);
                let index = match self.stack.pop().expect("ForNext with empty stack") {
                    Value::Int(i) if i >= 0 => i,
                    other => {
                        return Err(Unwind::Error(format!(
                            "for loop internal index must be non-negative int, got {}",
                            other.type_name()
                        )))
                    }
                };
                let iterable = self.stack.last().expect("ForNext missing iterable").clone();
                let items = iterable_values(&iterable)?;
                if let Some(item) = items.get(index as usize).cloned() {
                    self.frames
                        .last()
                        .unwrap()
                        .env
                        .borrow_mut()
                        .define(&name, item, true);
                    self.stack.push(Value::Int(index + 1));
                } else {
                    self.stack.pop();
                    self.jump(*off);
                }
            }
            Instr::BuildList(n) => {
                let n = *n as usize;
                let at = self.stack.len() - n;
                let items: Vec<Value> = self.stack.drain(at..).collect();
                self.stack.push(Value::List(Rc::new(RefCell::new(items))));
            }
            Instr::Index => {
                let i = self.stack.pop().unwrap();
                let t = self.stack.pop().unwrap();
                self.stack.push(index_value(&t, &i)?);
            }
            Instr::SetIndex => {
                let v = self.stack.pop().unwrap();
                let i = self.stack.pop().unwrap();
                let t = self.stack.pop().unwrap();
                match (&t, &i) {
                    (Value::List(xs), Value::Int(idx)) => {
                        let mut xs = xs.borrow_mut();
                        let len = xs.len() as i64;
                        let ix = if *idx < 0 { idx + len } else { *idx };
                        if ix < 0 || ix >= len {
                            return Err(Unwind::Error(format!(
                                "index {} out of bounds (len {})",
                                idx, len
                            )));
                        }
                        xs[ix as usize] = v.clone();
                    }
                    (Value::Map(m), Value::Str(k)) => {
                        m.borrow_mut().insert((**k).clone(), v.clone());
                    }
                    (Value::Bytes(bytes), Value::Int(idx)) => {
                        let byte = match v {
                            Value::Int(n) => u8::try_from(n)
                                .map_err(|_| Unwind::Error("byte value must be 0..=255".into()))?,
                            ref other => {
                                return Err(Unwind::Error(format!(
                                    "byte value must be int, got {}",
                                    other.type_name()
                                )))
                            }
                        };
                        let mut bytes = bytes.borrow_mut();
                        let len = bytes.len() as i64;
                        let ix = if *idx < 0 { idx + len } else { *idx };
                        if ix < 0 || ix >= len {
                            return Err(Unwind::Error(format!(
                                "index {} out of bounds (len {})",
                                idx, len
                            )));
                        }
                        bytes[ix as usize] = byte;
                        self.stack.push(Value::Int(byte as i64));
                        return Ok(());
                    }
                    _ => {
                        return Err(Unwind::Error(format!(
                            "cannot index-assign into {} with {}",
                            t.type_name(),
                            i.type_name()
                        )))
                    }
                }
                self.stack.push(v);
            }
            Instr::GetField(idx) => {
                let name = self.name(*idx);
                let t = self.stack.pop().unwrap();
                self.stack.push(field_value(&t, &name)?);
            }
            Instr::SetField(idx) => {
                let name = self.name(*idx);
                let v = self.stack.pop().unwrap();
                let t = self.stack.pop().unwrap();
                match t {
                    Value::Map(m) => {
                        m.borrow_mut().insert(name, v.clone());
                    }
                    other => {
                        return Err(Unwind::Error(format!(
                            "cannot set field `{}` on {}",
                            name,
                            other.type_name()
                        )))
                    }
                }
                self.stack.push(v);
            }
            Instr::Method(idx, argc) => {
                let name = self.name(*idx);
                let argc = *argc as usize;
                let at = self.stack.len() - argc;
                let args: Vec<Value> = self.stack.drain(at..).collect();
                let target = self.stack.pop().unwrap();
                let result = if let Value::Capability(c) = &target {
                    let c = c.clone();
                    crate::interp::call_capability_method(
                        &c,
                        &name,
                        &args,
                        self as &mut dyn Runtime,
                    )?
                } else {
                    call_method(&target, &name, &args)?
                };
                self.stack.push(result);
            }
            Instr::Call(argc) => {
                let argc = *argc as usize;
                let at = self.stack.len() - argc;
                let args: Vec<Value> = self.stack.drain(at..).collect();
                let callee = self.stack.pop().unwrap();
                self.dispatch_call(callee, args)?;
            }
            Instr::Return => {
                self.do_return();
            }
            Instr::MakeFn(idx) => {
                let proto = self.frames.last().unwrap().proto.chunk.protos[*idx as usize].clone();
                let closure = self.frames.last().unwrap().env.clone();
                let name = proto.name.clone();
                self.stack.push(Value::VmFn(Rc::new(VmFnObj {
                    proto,
                    closure,
                    name,
                })));
            }
            Instr::PushScope => {
                let f = self.frames.last_mut().unwrap();
                let child = Env::child(&f.env);
                f.env = child;
            }
            Instr::PopScope => {
                let f = self.frames.last_mut().unwrap();
                let parent = f
                    .env
                    .borrow()
                    .parent
                    .clone()
                    .expect("PopScope with no parent env");
                f.env = parent;
            }
            Instr::Panic => {
                let v = self.stack.pop().unwrap();
                return Err(Unwind::Panic(format!("panic: {}", v)));
            }
            Instr::Try => {
                let v = self.stack.pop().unwrap();
                match v {
                    Value::Result(r) => match r.as_ref() {
                        crate::value::ResultValue::Ok(inner) => self.stack.push(inner.clone()),
                        crate::value::ResultValue::Err(e) => return Err(Unwind::TryErr(e.clone())),
                    },
                    other => {
                        return Err(Unwind::Error(format!(
                            "? operator applied to {}, expected Result",
                            other.type_name()
                        )))
                    }
                }
            }
        }
        Ok(())
    }

    fn step(&mut self, instr: Instr, _code_len: usize) -> Result<(), Unwind> {
        match instr {
            Instr::Pop => {
                self.stack.pop();
            }

            Instr::Const(idx) => {
                let v = clone_const_value(
                    &self.frames.last().unwrap().proto.chunk.consts[idx as usize],
                );
                self.stack.push(v);
            }
            Instr::Nil => self.stack.push(Value::Nil),
            Instr::True => self.stack.push(Value::Bool(true)),
            Instr::False => self.stack.push(Value::Bool(false)),

            // --- Name-based variable access (slower path) ---
            Instr::GetName(idx) => {
                let name = self.name(idx);
                let env = self.frames.last().unwrap().env.clone();
                let v = env.borrow().get(&name)?;
                self.stack.push(v);
            }
            Instr::GetMove(idx) => {
                let name = self.name(idx);
                let env = self.frames.last().unwrap().env.clone();
                let v = env.borrow_mut().take(&name)?;
                self.stack.push(v);
            }
            Instr::DefLet(idx, mutable) => {
                let name = self.name(idx);
                let v = self.stack.pop().expect("DefLet with empty stack");
                self.frames
                    .last()
                    .unwrap()
                    .env
                    .borrow_mut()
                    .define(&name, v, mutable);
            }
            Instr::Assign(idx) => {
                let name = self.name(idx);
                let v = self.stack.pop().expect("Assign with empty stack");
                self.frames
                    .last()
                    .unwrap()
                    .env
                    .borrow_mut()
                    .assign(&name, v.clone())?;
                self.stack.push(v);
            }

            // --- Fast local slot access (O(1) indexed) ---
            Instr::GetLocal(idx) => {
                let v = self.get_local(idx as usize);
                self.stack.push(v);
            }
            Instr::SetLocal(idx) => {
                let v = self.stack.pop().expect("SetLocal with empty stack");
                self.set_local(idx as usize, v.clone());
                self.stack.push(v);
            }
            Instr::DefLocal(idx, _mutable) => {
                let v = self.stack.pop().expect("DefLocal with empty stack");
                self.set_local(idx as usize, v);
            }
            Instr::MoveLocal(idx) => {
                let v = self.take_local(idx as usize);
                self.stack.push(v);
            }

            Instr::Neg => {
                let v = self.stack.pop().unwrap();
                self.stack.push(apply_unary(crate::ast::UnOp::Neg, v)?);
            }
            Instr::Not => {
                let v = self.stack.pop().unwrap();
                self.stack.push(apply_unary(crate::ast::UnOp::Not, v)?);
            }

            Instr::Add => self.binary(crate::ast::BinOp::Add)?,
            Instr::Sub => self.binary(crate::ast::BinOp::Sub)?,
            Instr::Mul => self.binary(crate::ast::BinOp::Mul)?,
            Instr::Div => self.binary(crate::ast::BinOp::Div)?,
            Instr::Mod => self.binary(crate::ast::BinOp::Mod)?,
            Instr::Eq => self.binary(crate::ast::BinOp::Eq)?,
            Instr::NotEq => self.binary(crate::ast::BinOp::NotEq)?,
            Instr::Lt => self.binary(crate::ast::BinOp::Lt)?,
            Instr::Gt => self.binary(crate::ast::BinOp::Gt)?,
            Instr::LtEq => self.binary(crate::ast::BinOp::LtEq)?,
            Instr::GtEq => self.binary(crate::ast::BinOp::GtEq)?,

            Instr::Jump(off) => {
                self.jump(off);
            }
            Instr::JumpIfFalse(off) => {
                let v = self.stack.pop().unwrap();
                if !v.truthy() {
                    self.jump(off);
                }
            }
            Instr::JumpIfFalseKeep(off) => {
                let truthy = self.stack.last().unwrap().truthy();
                if !truthy {
                    self.jump(off);
                }
            }
            Instr::JumpIfTrueKeep(off) => {
                let truthy = self.stack.last().unwrap().truthy();
                if truthy {
                    self.jump(off);
                }
            }
            Instr::IterInit => {
                let iterable = self.stack.pop().expect("IterInit with empty stack");
                let items = iterable_values(&iterable)?;
                self.stack.push(Value::List(Rc::new(RefCell::new(items))));
            }
            Instr::ForNext(idx, off) => {
                let name = self.name(idx);
                let index = match self.stack.pop().expect("ForNext with empty stack") {
                    Value::Int(index) if index >= 0 => index,
                    other => {
                        return Err(Unwind::Error(format!(
                            "for loop internal index must be non-negative int, got {}",
                            other.type_name()
                        )))
                    }
                };
                let iterable = self.stack.last().expect("ForNext missing iterable").clone();
                let items = iterable_values(&iterable)?;
                if let Some(item) = items.get(index as usize).cloned() {
                    self.frames
                        .last()
                        .unwrap()
                        .env
                        .borrow_mut()
                        .define(&name, item, true);
                    self.stack.push(Value::Int(index + 1));
                } else {
                    self.stack.pop();
                    self.jump(off);
                }
            }

            Instr::BuildList(n) => {
                let n = n as usize;
                let at = self.stack.len() - n;
                let items: Vec<Value> = self.stack.drain(at..).collect();
                self.stack.push(Value::List(Rc::new(RefCell::new(items))));
            }
            Instr::Index => {
                let i = self.stack.pop().unwrap();
                let t = self.stack.pop().unwrap();
                self.stack.push(index_value(&t, &i)?);
            }
            Instr::SetIndex => {
                let v = self.stack.pop().unwrap();
                let i = self.stack.pop().unwrap();
                let t = self.stack.pop().unwrap();
                match (&t, &i) {
                    (Value::List(xs), Value::Int(idx)) => {
                        let mut xs = xs.borrow_mut();
                        let len = xs.len() as i64;
                        let ix = if *idx < 0 { idx + len } else { *idx };
                        if ix < 0 || ix >= len {
                            return Err(Unwind::Error(format!(
                                "index {} out of bounds (len {})",
                                idx, len
                            )));
                        }
                        xs[ix as usize] = v.clone();
                    }
                    (Value::Map(m), Value::Str(k)) => {
                        m.borrow_mut().insert((**k).clone(), v.clone());
                    }
                    (Value::Bytes(bytes), Value::Int(idx)) => {
                        let byte = match v {
                            Value::Int(n) => u8::try_from(n).map_err(|_| {
                                Unwind::Error("byte assignment value must be in 0..=255".into())
                            })?,
                            ref other => {
                                return Err(Unwind::Error(format!(
                                    "byte assignment value must be int, got {}",
                                    other.type_name()
                                )))
                            }
                        };
                        let mut bytes = bytes.borrow_mut();
                        let len = bytes.len() as i64;
                        let ix = if *idx < 0 { idx + len } else { *idx };
                        if ix < 0 || ix >= len {
                            return Err(Unwind::Error(format!(
                                "index {} out of bounds (len {})",
                                idx, len
                            )));
                        }
                        bytes[ix as usize] = byte;
                        self.stack.push(Value::Int(byte as i64));
                        return Ok(());
                    }
                    _ => {
                        return Err(Unwind::Error(format!(
                            "cannot index-assign into {} with {}",
                            t.type_name(),
                            i.type_name()
                        )))
                    }
                }
                self.stack.push(v);
            }
            Instr::GetField(idx) => {
                let name = self.name(idx);
                let t = self.stack.pop().unwrap();
                self.stack.push(field_value(&t, &name)?);
            }
            Instr::SetField(idx) => {
                let name = self.name(idx);
                let v = self.stack.pop().unwrap();
                let t = self.stack.pop().unwrap();
                match t {
                    Value::Map(m) => {
                        m.borrow_mut().insert(name, v.clone());
                    }
                    other => {
                        return Err(Unwind::Error(format!(
                            "cannot set field `{}` on {}",
                            name,
                            other.type_name()
                        )))
                    }
                }
                self.stack.push(v);
            }
            Instr::Method(idx, argc) => {
                let name = self.name(idx);
                let argc = argc as usize;
                let at = self.stack.len() - argc;
                let args: Vec<Value> = self.stack.drain(at..).collect();
                let target = self.stack.pop().unwrap();
                let result = if let Value::Capability(c) = &target {
                    let c = c.clone();
                    crate::interp::call_capability_method(
                        &c,
                        &name,
                        &args,
                        self as &mut dyn Runtime,
                    )?
                } else {
                    call_method(&target, &name, &args)?
                };
                self.stack.push(result);
            }

            Instr::Call(argc) => {
                let argc = argc as usize;
                let at = self.stack.len() - argc;
                let args: Vec<Value> = self.stack.drain(at..).collect();
                let callee = self.stack.pop().unwrap();
                self.dispatch_call(callee, args)?;
            }

            Instr::Return => {
                self.do_return();
            }

            Instr::MakeFn(idx) => {
                let proto = self.frames.last().unwrap().proto.chunk.protos[idx as usize].clone();
                let closure = self.frames.last().unwrap().env.clone();
                let name = proto.name.clone();
                self.stack.push(Value::VmFn(Rc::new(VmFnObj {
                    proto,
                    closure,
                    name,
                })));
            }

            Instr::PushScope => {
                let f = self.frames.last_mut().unwrap();
                let child = Env::child(&f.env);
                f.env = child;
            }
            Instr::PopScope => {
                let f = self.frames.last_mut().unwrap();
                let parent = f
                    .env
                    .borrow()
                    .parent
                    .clone()
                    .expect("PopScope with no parent env (compiler bug)");
                f.env = parent;
            }

            Instr::Panic => {
                let v = self.stack.pop().unwrap();
                return Err(Unwind::Panic(format!("panic: {}", v)));
            }

            Instr::Try => {
                let v = self.stack.pop().unwrap();
                match v {
                    Value::Result(r) => match r.as_ref() {
                        crate::value::ResultValue::Ok(inner) => self.stack.push(inner.clone()),
                        crate::value::ResultValue::Err(e) => return Err(Unwind::TryErr(e.clone())),
                    },
                    other => {
                        return Err(Unwind::Error(format!(
                            "? operator applied to {}, expected Result",
                            other.type_name()
                        )))
                    }
                }
            }
        }
        Ok(())
    }

    /// Get a value from the fast local slots array (clone, like Env::get).
    fn get_local(&self, idx: usize) -> Value {
        let f = self.frames.last().unwrap();
        if idx < f.locals.len() {
            f.locals[idx].clone()
        } else {
            Value::Nil
        }
    }

    /// Set a value in the fast local slots array.
    fn set_local(&mut self, idx: usize, value: Value) {
        let f = self.frames.last_mut().unwrap();
        if idx < f.locals.len() {
            f.locals[idx] = value;
        }
    }

    /// Take (move) a value from the fast local slots array.
    /// For Copy values, clones. For heap values, takes and leaves Nil.
    fn take_local(&mut self, idx: usize) -> Value {
        let f = self.frames.last_mut().unwrap();
        if idx < f.locals.len() {
            std::mem::replace(&mut f.locals[idx], Value::Nil)
        } else {
            Value::Nil
        }
    }

    fn do_return(&mut self) {
        let val = self.stack.pop().unwrap_or(Value::Nil);
        self.frames.pop();
        if !self.frames.is_empty() {
            self.stack.push(val);
        }
    }

    fn jump(&mut self, off: i32) {
        let f = self.frames.last_mut().unwrap();
        let new_ip = f.ip as i64 + off as i64;
        debug_assert!(new_ip >= 0, "negative ip after jump");
        f.ip = new_ip as usize;
    }

    fn binary(&mut self, op: crate::ast::BinOp) -> Result<(), Unwind> {
        let r = self.stack.pop().unwrap();
        let l = self.stack.pop().unwrap();
        self.stack.push(apply_binary(op, l, r)?);
        Ok(())
    }

    fn name(&self, idx: u16) -> String {
        self.frames.last().unwrap().proto.chunk.names[idx as usize].clone()
    }

    fn dispatch_call(&mut self, callee: Value, args: Vec<Value>) -> Result<(), Unwind> {
        match callee {
            Value::VmFn(f) => {
                if args.len() != f.proto.params.len() {
                    return Err(Unwind::Error(format!(
                        "{} expected {} args, got {}",
                        f.name.as_deref().unwrap_or("<fn>"),
                        f.proto.params.len(),
                        args.len(),
                    )));
                }
                let scope = Env::child(&f.closure);
                {
                    let mut env = scope.borrow_mut();
                    for (name, val) in f.proto.params.iter().zip(args.into_iter()) {
                        env.define(name, val, true);
                    }
                }
                let local_count = f.proto.chunk.local_count as usize;
                self.frames.push(Frame {
                    proto: f.proto.clone(),
                    ip: 0,
                    env: scope,
                    locals: vec![Value::Nil; local_count],
                });
                Ok(())
            }
            Value::Native(n) => {
                if let Some(arity) = n.arity {
                    if args.len() != arity {
                        return Err(Unwind::Error(format!(
                            "{} expected {} args, got {}",
                            n.name,
                            arity,
                            args.len()
                        )));
                    }
                }
                let result = match &n.func {
                    NativeFunc::Pure(f) => f(&args).map_err(Unwind::Error)?,
                    NativeFunc::Runtime(f) => {
                        f(self as &mut dyn Runtime, &args).map_err(Unwind::Error)?
                    }
                };
                self.stack.push(result);
                Ok(())
            }
            Value::Fn(_) => Err(Unwind::Error(
                "tree-walker fn reached the VM (internal inconsistency)".into(),
            )),
            other => Err(Unwind::Error(format!(
                "{} is not callable",
                other.type_name()
            ))),
        }
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

pub struct VmBuilder {
    instruction_budget: Option<u64>,
    grants: Vec<(String, Rc<dyn crate::capability::Authority>)>,
}

impl VmBuilder {
    pub fn new() -> Self {
        Self {
            instruction_budget: None,
            grants: Vec::new(),
        }
    }

    pub fn instruction_limit(mut self, limit: u64) -> Self {
        self.instruction_budget = Some(limit);
        self
    }

    pub fn capability(
        mut self,
        name: impl Into<String>,
        authority: Rc<dyn crate::capability::Authority>,
    ) -> Self {
        self.grants.push((name.into(), authority));
        self
    }

    pub fn build(self) -> VM {
        let mut vm = VM::new_with_budget(self.instruction_budget);
        for (name, authority) in self.grants {
            vm.grant(&name, authority);
        }
        vm
    }
}

impl Default for VmBuilder {
    fn default() -> Self {
        Self::new()
    }
}

fn format_unwind(u: Unwind) -> String {
    match u {
        Unwind::Error(e) | Unwind::Panic(e) | Unwind::TryErr(e) => e,
    }
}

/// Lets runtime-aware natives synchronously invoke a VM callable.
///
/// For `Value::VmFn` callees, `dispatch_call` pushes a new frame and we drive
/// the interpreter loop until the frame count is back to where we started.
/// For native callees, `dispatch_call` already pushed the result onto the
/// stack, so no extra work is needed.
fn clone_const_value(value: &Value) -> Value {
    match value {
        Value::Bytes(bytes) => Value::Bytes(Rc::new(RefCell::new(bytes.borrow().clone()))),
        other => other.clone(),
    }
}

impl Runtime for VM {
    fn invoke(&mut self, callee: &Value, args: &[Value]) -> Result<Value, String> {
        let depth = self.frames.len();
        if let Err(u) = self.dispatch_call(callee.clone(), args.to_vec()) {
            return Err(format_unwind(u));
        }
        while self.frames.len() > depth {
            let (instr, code_len) = {
                let f = self.frames.last().unwrap();
                let code_len = f.proto.chunk.code.len();
                if f.ip >= code_len {
                    self.stack.push(Value::Nil);
                    self.do_return();
                    continue;
                }
                (f.proto.chunk.code[f.ip].clone(), code_len)
            };
            self.frames.last_mut().unwrap().ip += 1;
            match self.step(instr, code_len) {
                Ok(()) => {}
                Err(Unwind::TryErr(e)) => {
                    self.stack
                        .push(Value::Result(Rc::new(crate::value::ResultValue::Err(e))));
                    self.do_return();
                }
                Err(u) => return Err(format_unwind(u)),
            }
        }
        Ok(self.stack.pop().unwrap_or(Value::Nil))
    }
}
