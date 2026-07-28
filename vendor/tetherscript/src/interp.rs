//! Tree-walking interpreter.
//!
//! Evaluates the AST directly. Slow compared to a bytecode VM, but a great
//! reference implementation: easy to reason about, easy to debug, and gives
//! us a runnable language *today*. The bytecode VM will port these semantics
//! one-to-one.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fmt::Write as _;
use std::rc::Rc;

use crate::ast::*;
use crate::browser;
use crate::browser_js;
use crate::http;
use crate::js;
use crate::json;
use crate::lexer::Lexer;
use crate::output;
use crate::parser::Parser;
use crate::smtp;
use crate::system;
use crate::value::{Env, FnObj, NativeFn, NativeFunc, Runtime, Slot, Value};

/// Non-local control flow. Wrapped in Result::Err so we can `?` it through
/// the evaluator without polluting the happy path.
pub enum Unwind {
    Error(String),
    Return(Value),
    Panic(String),
    /// Short-circuit from `expr?` when expr is Err(_). Caught by the
    /// enclosing function call, which converts it to a `Return(Err(...))`.
    TryErr(String),
}

impl From<String> for Unwind {
    fn from(s: String) -> Self {
        Unwind::Error(s)
    }
}

pub type EvalResult = Result<Value, Unwind>;

pub struct Interpreter {
    pub globals: Rc<RefCell<Env>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Env::new_global();
        install_builtins(&globals);
        Self { globals }
    }

    /// Interpreter with a restricted built-in set: no `http_serve`, no `eval`,
    /// and no potentially expensive browser parsing/layout/rendering natives.
    /// Used to run untrusted source passed to `eval(...)` from a hosted REPL.
    pub fn new_sandboxed() -> Self {
        let globals = Env::new_global();
        install_sandbox_builtins(&globals);
        Self { globals }
    }

    /// Install a capability at the given global name. The harness calls this
    /// to grant authority to the agent: `interp.grant("fs", FsAuthority::new(root))`.
    /// Grants are additive — calling grant twice with the same name replaces
    /// the binding.
    pub fn grant(&mut self, name: &str, authority: Rc<dyn crate::capability::Authority>) {
        let cap = crate::capability::Capability::new_root(name, authority);
        self.globals
            .borrow_mut()
            .define(name, Value::Capability(cap), false);
    }

    /// Run a program as a REPL snippet: hoist fn decls, evaluate each
    /// top-level statement in order, and return the last evaluated value
    /// (or `Nil` if the snippet ended in a statement with no value).
    /// Unlike `run`, this does NOT auto-invoke `main`.
    pub fn run_repl(&mut self, program: &Program) -> Result<Value, String> {
        for stmt in &program.stmts {
            if let Stmt::FnDecl { name, params, body } = stmt {
                let func = Value::Fn(Rc::new(FnObj {
                    params: params.clone(),
                    body: body.clone(),
                    closure: self.globals.clone(),
                    name: Some(name.clone()),
                }));
                self.globals.borrow_mut().define(name, func, false);
            }
        }
        let mut last = Value::Nil;
        for stmt in &program.stmts {
            if matches!(stmt, Stmt::FnDecl { .. }) {
                continue;
            }
            match self.exec_stmt(stmt, &self.globals.clone()) {
                Ok(v) => last = v,
                Err(Unwind::Error(e)) | Err(Unwind::Panic(e)) => return Err(e),
                Err(Unwind::Return(_)) => return Err("`return` outside of function".into()),
                Err(Unwind::TryErr(e)) => return Err(format!("unhandled `?` error: {}", e)),
            }
        }
        Ok(last)
    }

    pub fn run(&mut self, program: &Program) -> Result<(), String> {
        // Two-pass: hoist top-level fn declarations so forward references
        // (e.g. `main` calling `fib` defined below) work.
        for stmt in &program.stmts {
            if let Stmt::FnDecl { name, params, body } = stmt {
                let func = Value::Fn(Rc::new(FnObj {
                    params: params.clone(),
                    body: body.clone(),
                    closure: self.globals.clone(),
                    name: Some(name.clone()),
                }));
                self.globals.borrow_mut().define(name, func, false);
            }
        }

        for stmt in &program.stmts {
            if matches!(stmt, Stmt::FnDecl { .. }) {
                continue;
            }
            match self.exec_stmt(stmt, &self.globals.clone()) {
                Ok(_) => {}
                Err(Unwind::Error(e)) | Err(Unwind::Panic(e)) => return Err(e),
                Err(Unwind::Return(_)) => return Err("`return` outside of function".into()),
                Err(Unwind::TryErr(e)) => {
                    return Err(format!("unhandled `?` error at top level: {}", e))
                }
            }
        }

        // Conventional entry point: if `main` is defined, call it.
        let has_main = self.globals.borrow().slots.contains_key("main");
        if has_main {
            let main = self.globals.borrow().get("main")?;
            match self.call(&main, &[]) {
                Ok(_) => Ok(()),
                Err(Unwind::Error(e)) | Err(Unwind::Panic(e)) => Err(e),
                Err(Unwind::Return(_)) => Err("`return` unwound out of main".into()),
                Err(Unwind::TryErr(e)) => Err(format!("unhandled `?` error from main: {}", e)),
            }
        } else {
            Ok(())
        }
    }

    // ---------- statements ----------

    fn exec_stmt(&self, stmt: &Stmt, env: &Rc<RefCell<Env>>) -> EvalResult {
        match stmt {
            Stmt::Let {
                name,
                mutable,
                value,
            } => {
                let v = self.eval(value, env)?;
                env.borrow_mut().define(name, v, *mutable);
                Ok(Value::Nil)
            }
            Stmt::Expr { expr, .. } => self.eval(expr, env),
            Stmt::FnDecl { name, params, body } => {
                let func = Value::Fn(Rc::new(FnObj {
                    params: params.clone(),
                    body: body.clone(),
                    closure: env.clone(),
                    name: Some(name.clone()),
                }));
                env.borrow_mut().define(name, func, false);
                Ok(Value::Nil)
            }
        }
    }

    fn exec_block(&self, block: &Block, env: &Rc<RefCell<Env>>) -> EvalResult {
        let mut last = Value::Nil;
        for (i, stmt) in block.stmts.iter().enumerate() {
            let is_last = i == block.stmts.len() - 1;
            match stmt {
                Stmt::Expr { expr, terminated } => {
                    let v = self.eval(expr, env)?;
                    if is_last && !terminated {
                        last = v;
                    } else {
                        last = Value::Nil;
                    }
                }
                _ => {
                    self.exec_stmt(stmt, env)?;
                    last = Value::Nil;
                }
            }
        }
        Ok(last)
    }

    // ---------- expressions ----------

    fn eval(&self, expr: &Expr, env: &Rc<RefCell<Env>>) -> EvalResult {
        step_tick()?;
        match expr {
            Expr::Int(n) => Ok(Value::Int(*n)),
            Expr::Float(n) => Ok(Value::Float(*n)),
            Expr::Str(s) => Ok(Value::Str(Rc::new(s.clone()))),
            Expr::Bytes(bytes) => Ok(Value::Bytes(Rc::new(RefCell::new(bytes.clone())))),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::Nil => Ok(Value::Nil),

            Expr::Ident(name) => Ok(env.borrow().get(name)?),

            Expr::Move(inner) => {
                // `move x` — only meaningful for plain identifiers.
                match inner.as_ref() {
                    Expr::Ident(name) => Ok(env.borrow_mut().take(name)?),
                    other => {
                        // `move <expr>` on a non-identifier is just the value.
                        // Nothing to tombstone.
                        self.eval(other, env)
                    }
                }
            }

            // v0: borrows are implicit. `&x` and `&mut x` just evaluate to x.
            // The distinction will matter once we implement borrow tracking
            // on aggregate values (list mutation during iteration, etc.).
            Expr::Borrow(inner) | Expr::BorrowMut(inner) => self.eval(inner, env),

            Expr::Unary { op, rhs } => {
                let v = self.eval(rhs, env)?;
                apply_unary(*op, v).map_err(Unwind::Error)
            }

            Expr::Binary { op, lhs, rhs } => {
                if *op == BinOp::Assign {
                    return self.eval_assign(lhs, rhs, env);
                }
                if *op == BinOp::And {
                    let l = self.eval(lhs, env)?;
                    if !l.truthy() {
                        return Ok(l);
                    }
                    return self.eval(rhs, env);
                }
                if *op == BinOp::Or {
                    let l = self.eval(lhs, env)?;
                    if l.truthy() {
                        return Ok(l);
                    }
                    return self.eval(rhs, env);
                }
                let l = self.eval(lhs, env)?;
                let r = self.eval(rhs, env)?;
                apply_binary(*op, l, r).map_err(Unwind::Error)
            }

            Expr::List(items) => {
                let mut xs = Vec::with_capacity(items.len());
                for it in items {
                    xs.push(self.eval(it, env)?);
                }
                Ok(Value::List(Rc::new(RefCell::new(xs))))
            }

            Expr::Call { callee, args } => {
                let callee = self.eval(callee, env)?;
                let mut arg_vals = Vec::with_capacity(args.len());
                for a in args {
                    arg_vals.push(self.eval(a, env)?);
                }
                self.call(&callee, &arg_vals)
            }

            Expr::Index { target, index } => {
                let t = self.eval(target, env)?;
                let i = self.eval(index, env)?;
                index_value(&t, &i).map_err(Unwind::Error)
            }

            Expr::Field { target, name } => {
                let t = self.eval(target, env)?;
                field_value(&t, name).map_err(Unwind::Error)
            }

            Expr::Method { target, name, args } => {
                let t = self.eval(target, env)?;
                let mut arg_vals = Vec::with_capacity(args.len());
                for a in args {
                    arg_vals.push(self.eval(a, env)?);
                }
                // Capabilities dispatch through their Authority trait and
                // need a Runtime so they can invoke TetherScript callables. All
                // other method calls go through the flat `call_method`.
                if let Value::Capability(c) = &t {
                    let mut rt = InterpRuntime { interp: self };
                    return call_capability_method(c, name, &arg_vals, &mut rt)
                        .map_err(Unwind::Error);
                }
                call_method(&t, name, &arg_vals).map_err(Unwind::Error)
            }

            Expr::If {
                cond,
                then_branch,
                else_branch,
            } => {
                let c = self.eval(cond, env)?;
                if c.truthy() {
                    let scope = Env::child(env);
                    self.exec_block(then_branch, &scope)
                } else if let Some(else_branch) = else_branch {
                    let scope = Env::child(env);
                    self.exec_block(else_branch, &scope)
                } else {
                    Ok(Value::Nil)
                }
            }

            Expr::While { cond, body } => {
                loop {
                    let c = self.eval(cond, env)?;
                    if !c.truthy() {
                        break;
                    }
                    let scope = Env::child(env);
                    self.exec_block(body, &scope)?;
                }
                Ok(Value::Nil)
            }

            Expr::For { name, iter, body } => {
                let items = iterable_values(&self.eval(iter, env)?)?;
                for item in items {
                    let scope = Env::child(env);
                    scope.borrow_mut().define(name, item, true);
                    self.exec_block(body, &scope)?;
                }
                Ok(Value::Nil)
            }

            Expr::Block(block) => {
                let scope = Env::child(env);
                self.exec_block(block, &scope)
            }

            Expr::Fn { params, body } => Ok(Value::Fn(Rc::new(FnObj {
                params: params.clone(),
                body: body.clone(),
                closure: env.clone(),
                name: None,
            }))),

            Expr::Return(inner) => {
                let v = match inner {
                    Some(e) => self.eval(e, env)?,
                    None => Value::Nil,
                };
                Err(Unwind::Return(v))
            }

            Expr::Panic(msg) => {
                let v = self.eval(msg, env)?;
                Err(Unwind::Panic(format!("panic: {}", v)))
            }

            Expr::AsyncFn { params, body } => Ok(Value::Fn(Rc::new(FnObj {
                params: params.clone(),
                body: body.clone(),
                closure: env.clone(),
                name: None,
            }))),

            Expr::Await(inner) | Expr::Spawn(inner) => self.eval(inner, env),

            Expr::Join(exprs) => {
                let mut out = Vec::with_capacity(exprs.len());
                for expr in exprs {
                    out.push(self.eval(expr, env)?);
                }
                Ok(Value::List(Rc::new(RefCell::new(out))))
            }

            Expr::Try(inner) => {
                let v = self.eval(inner, env)?;
                match v {
                    Value::Result(r) => match r.as_ref() {
                        crate::value::ResultValue::Ok(inner) => Ok(inner.clone()),
                        crate::value::ResultValue::Err(e) => Err(Unwind::TryErr(e.clone())),
                    },
                    other => Err(Unwind::Error(format!(
                        "? operator applied to {}, expected Result",
                        other.type_name()
                    ))),
                }
            }
        }
    }

    fn eval_assign(&self, lhs: &Expr, rhs: &Expr, env: &Rc<RefCell<Env>>) -> EvalResult {
        let value = self.eval(rhs, env)?;
        match lhs {
            Expr::Ident(name) => {
                env.borrow_mut().assign(name, value.clone())?;
                Ok(value)
            }
            Expr::Index { target, index } => {
                let t = self.eval(target, env)?;
                let i = self.eval(index, env)?;
                match (&t, &i) {
                    (Value::List(xs), Value::Int(idx)) => {
                        let mut xs = xs.borrow_mut();
                        let len = xs.len() as i64;
                        let idx = if *idx < 0 { idx + len } else { *idx };
                        if idx < 0 || idx >= len {
                            return Err(Unwind::Error(format!(
                                "index {} out of bounds (len {})",
                                idx, len
                            )));
                        }
                        xs[idx as usize] = value.clone();
                        Ok(value)
                    }
                    (Value::Map(m), Value::Str(k)) => {
                        m.borrow_mut().insert((**k).clone(), value.clone());
                        Ok(value)
                    }
                    (Value::Bytes(bytes), Value::Int(idx)) => {
                        let byte = match value {
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
                        let idx = if *idx < 0 { idx + len } else { *idx };
                        if idx < 0 || idx >= len {
                            return Err(Unwind::Error(format!(
                                "index {} out of bounds (len {})",
                                idx, len
                            )));
                        }
                        bytes[idx as usize] = byte;
                        Ok(Value::Int(byte as i64))
                    }
                    _ => Err(Unwind::Error(format!(
                        "cannot index-assign into {} with {}",
                        t.type_name(),
                        i.type_name()
                    ))),
                }
            }
            Expr::Field { target, name } => {
                let t = self.eval(target, env)?;
                match t {
                    Value::Map(m) => {
                        m.borrow_mut().insert(name.clone(), value.clone());
                        Ok(value)
                    }
                    _ => Err(Unwind::Error(format!(
                        "cannot set field `{}` on {}",
                        name,
                        t.type_name()
                    ))),
                }
            }
            _ => Err(Unwind::Error("invalid assignment target".into())),
        }
    }

    pub fn call(&self, callee: &Value, args: &[Value]) -> EvalResult {
        match callee {
            Value::Fn(f) => {
                if args.len() != f.params.len() {
                    return Err(Unwind::Error(format!(
                        "{} expected {} args, got {}",
                        f.name.as_deref().unwrap_or("<fn>"),
                        f.params.len(),
                        args.len()
                    )));
                }
                let scope = Env::child(&f.closure);
                {
                    let mut s = scope.borrow_mut();
                    for (name, val) in f.params.iter().zip(args.iter()) {
                        s.slots.insert(
                            name.clone(),
                            Slot::Live {
                                value: val.clone(),
                                mutable: true,
                            },
                        );
                    }
                }
                match self.exec_block(&f.body, &scope) {
                    Ok(v) => Ok(v),
                    Err(Unwind::Return(v)) => Ok(v),
                    // `expr?` inside `f` short-circuited with an Err — lift
                    // it to the function's return value so callers see an
                    // Err Result, not a runtime error.
                    Err(Unwind::TryErr(e)) => {
                        Ok(Value::Result(Rc::new(crate::value::ResultValue::Err(e))))
                    }
                    Err(other) => Err(other),
                }
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
                match &n.func {
                    NativeFunc::Pure(f) => f(args).map_err(Unwind::Error),
                    NativeFunc::Runtime(f) => {
                        let mut rt = InterpRuntime { interp: self };
                        f(&mut rt, args).map_err(Unwind::Error)
                    }
                }
            }
            other => Err(Unwind::Error(format!(
                "{} is not callable",
                other.type_name()
            ))),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

// ---------- operators ----------

pub(crate) fn apply_unary(op: UnOp, v: Value) -> Result<Value, String> {
    match (op, v) {
        (UnOp::Neg, Value::Int(n)) => Ok(Value::Int(-n)),
        (UnOp::Neg, Value::Float(n)) => Ok(Value::Float(-n)),
        (UnOp::Not, v) => Ok(Value::Bool(!v.truthy())),
        (op, v) => Err(format!("cannot apply {:?} to {}", op, v.type_name())),
    }
}

pub(crate) fn apply_binary(op: BinOp, l: Value, r: Value) -> Result<Value, String> {
    use BinOp::*;
    use Value::*;
    match (op, &l, &r) {
        // Numeric
        (Add, Int(a), Int(b)) => Ok(Int(a + b)),
        (Sub, Int(a), Int(b)) => Ok(Int(a - b)),
        (Mul, Int(a), Int(b)) => Ok(Int(a * b)),
        (Div, Int(_), Int(0)) => Err("integer division by zero".into()),
        (Div, Int(a), Int(b)) => Ok(Int(a / b)),
        (Mod, Int(_), Int(0)) => Err("integer modulo by zero".into()),
        (Mod, Int(a), Int(b)) => Ok(Int(a % b)),

        (Add, Float(a), Float(b)) => Ok(Float(a + b)),
        (Sub, Float(a), Float(b)) => Ok(Float(a - b)),
        (Mul, Float(a), Float(b)) => Ok(Float(a * b)),
        (Div, Float(a), Float(b)) => Ok(Float(a / b)),

        // String concatenation
        (Add, Str(a), Str(b)) => Ok(Str(Rc::new(format!("{}{}", a, b)))),
        (Add, Str(a), b) => Ok(Str(Rc::new(format!("{}{}", a, b)))),
        (Add, a, Str(b)) => Ok(Str(Rc::new(format!("{}{}", a, b)))),

        // Comparisons
        (Eq, a, b) => Ok(Bool(values_eq(a, b))),
        (NotEq, a, b) => Ok(Bool(!values_eq(a, b))),
        (Lt, Int(a), Int(b)) => Ok(Bool(a < b)),
        (Gt, Int(a), Int(b)) => Ok(Bool(a > b)),
        (LtEq, Int(a), Int(b)) => Ok(Bool(a <= b)),
        (GtEq, Int(a), Int(b)) => Ok(Bool(a >= b)),
        (Lt, Float(a), Float(b)) => Ok(Bool(a < b)),
        (Gt, Float(a), Float(b)) => Ok(Bool(a > b)),
        (LtEq, Float(a), Float(b)) => Ok(Bool(a <= b)),
        (GtEq, Float(a), Float(b)) => Ok(Bool(a >= b)),

        (op, a, b) => Err(format!(
            "cannot apply {:?} to {} and {}",
            op,
            a.type_name(),
            b.type_name()
        )),
    }
}

fn values_eq(a: &Value, b: &Value) -> bool {
    use Value::*;
    match (a, b) {
        (Nil, Nil) => true,
        (Int(a), Int(b)) => a == b,
        (Float(a), Float(b)) => a == b,
        (Int(a), Float(b)) => (*a as f64) == *b,
        (Float(a), Int(b)) => *a == (*b as f64),
        (Bool(a), Bool(b)) => a == b,
        (Bytes(a), Bytes(b)) => *a.borrow() == *b.borrow(),
        (Str(a), Str(b)) => a == b,
        _ => false,
    }
}

pub(crate) fn index_value(target: &Value, index: &Value) -> Result<Value, String> {
    match (target, index) {
        (Value::List(xs), Value::Int(i)) => {
            let xs = xs.borrow();
            let len = xs.len() as i64;
            let idx = if *i < 0 { i + len } else { *i };
            if idx < 0 || idx >= len {
                return Err(format!("index {} out of bounds (len {})", i, len));
            }
            Ok(xs[idx as usize].clone())
        }
        (Value::Map(m), Value::Str(k)) => Ok(m.borrow().get(&**k).cloned().unwrap_or(Value::Nil)),
        (Value::Str(s), Value::Int(i)) => {
            let bytes = s.as_bytes();
            let len = bytes.len() as i64;
            let idx = if *i < 0 { i + len } else { *i };
            if idx < 0 || idx >= len {
                return Err(format!("index {} out of bounds (len {})", i, len));
            }
            Ok(Value::Str(Rc::new(
                (bytes[idx as usize] as char).to_string(),
            )))
        }
        (Value::Bytes(bytes), Value::Int(i)) => {
            let bytes = bytes.borrow();
            let len = bytes.len() as i64;
            let idx = if *i < 0 { i + len } else { *i };
            if idx < 0 || idx >= len {
                return Err(format!("index {} out of bounds (len {})", i, len));
            }
            Ok(Value::Int(bytes[idx as usize] as i64))
        }
        (t, i) => Err(format!(
            "cannot index {} with {}",
            t.type_name(),
            i.type_name()
        )),
    }
}

pub(crate) fn field_value(target: &Value, name: &str) -> Result<Value, String> {
    match target {
        Value::Map(m) => Ok(m.borrow().get(name).cloned().unwrap_or(Value::Nil)),
        _ => Err(format!(
            "cannot access field `{}` on {}",
            name,
            target.type_name()
        )),
    }
}

pub(crate) fn iterable_values(value: &Value) -> Result<Vec<Value>, String> {
    match value {
        Value::List(items) => Ok(items.borrow().iter().cloned().collect()),
        Value::Str(text) => Ok(text
            .chars()
            .map(|ch| Value::Str(Rc::new(ch.to_string())))
            .collect()),
        Value::Bytes(bytes) => Ok(bytes
            .borrow()
            .iter()
            .map(|b| Value::Int(*b as i64))
            .collect()),
        Value::Map(map) => {
            let mut keys: Vec<String> = map.borrow().keys().cloned().collect();
            keys.sort();
            Ok(keys
                .into_iter()
                .map(|key| Value::Str(Rc::new(key)))
                .collect())
        }
        other => Err(format!(
            "for loop cannot iterate over {}",
            other.type_name()
        )),
    }
}

pub(crate) fn call_method(target: &Value, name: &str, args: &[Value]) -> Result<Value, String> {
    match (target, name, args) {
        (Value::List(xs), "len", []) => Ok(Value::Int(xs.borrow().len() as i64)),
        (Value::List(xs), "push", [v]) => {
            xs.borrow_mut().push(v.clone());
            Ok(Value::Nil)
        }
        (Value::List(xs), "pop", []) => Ok(xs.borrow_mut().pop().unwrap_or(Value::Nil)),
        (Value::List(xs), "join", [sep]) => {
            let sep = match sep {
                Value::Str(sep) => sep.as_str(),
                other => {
                    return Err(format!(
                        "list.join: separator must be str, got {}",
                        other.type_name()
                    ))
                }
            };
            let parts: Vec<String> = xs.borrow().iter().map(|value| value.to_string()).collect();
            Ok(Value::Str(Rc::new(parts.join(sep))))
        }
        (Value::List(xs), "contains", [needle]) => {
            Ok(Value::Bool(xs.borrow().iter().any(|value| value == needle)))
        }
        (Value::Bytes(bytes), "len", []) => Ok(Value::Int(bytes.borrow().len() as i64)),
        (Value::Bytes(bytes), "push", [Value::Int(byte)]) => {
            let byte = u8::try_from(*byte)
                .map_err(|_| "bytes.push: value must be in 0..=255".to_string())?;
            bytes.borrow_mut().push(byte);
            Ok(Value::Nil)
        }
        (Value::Bytes(bytes), "pop", []) => Ok(bytes
            .borrow_mut()
            .pop()
            .map(|b| Value::Int(b as i64))
            .unwrap_or(Value::Nil)),
        (Value::Bytes(bytes), "decode_utf8", []) | (Value::Bytes(bytes), "to_string", []) => {
            let text = String::from_utf8(bytes.borrow().clone())
                .map_err(|e| format!("bytes.decode_utf8: {}", e))?;
            Ok(Value::Str(Rc::new(text)))
        }
        (Value::Bytes(bytes), "hex", []) => {
            let bytes = bytes.borrow();
            let mut out = String::with_capacity(bytes.len() * 2);
            for b in bytes.iter() {
                write!(&mut out, "{:02x}", b).expect("writing to String cannot fail");
            }
            Ok(Value::Str(Rc::new(out)))
        }
        (Value::Bytes(_), "push", [other]) => Err(format!(
            "bytes.push: value must be int, got {}",
            other.type_name()
        )),
        (Value::Str(s), "len", []) => Ok(Value::Int(s.len() as i64)),
        (Value::Str(s), "upper", []) => Ok(Value::Str(Rc::new(s.to_uppercase()))),
        (Value::Str(s), "lower", []) => Ok(Value::Str(Rc::new(s.to_lowercase()))),
        (Value::Str(s), "trim", []) => Ok(Value::Str(Rc::new(s.trim().to_string()))),
        (Value::Str(s), "contains", [needle]) => {
            let needle = match needle {
                Value::Str(needle) => needle.as_str(),
                other => {
                    return Err(format!(
                        "str.contains: needle must be str, got {}",
                        other.type_name()
                    ))
                }
            };
            Ok(Value::Bool(s.contains(needle)))
        }
        (Value::Str(s), "starts_with", [needle]) => {
            let needle = match needle {
                Value::Str(needle) => needle.as_str(),
                other => {
                    return Err(format!(
                        "str.starts_with: needle must be str, got {}",
                        other.type_name()
                    ))
                }
            };
            Ok(Value::Bool(s.starts_with(needle)))
        }
        (Value::Str(s), "ends_with", [needle]) => {
            let needle = match needle {
                Value::Str(needle) => needle.as_str(),
                other => {
                    return Err(format!(
                        "str.ends_with: needle must be str, got {}",
                        other.type_name()
                    ))
                }
            };
            Ok(Value::Bool(s.ends_with(needle)))
        }
        (Value::Str(s), "replace", [from, to]) => {
            let from = match from {
                Value::Str(from) => from.as_str(),
                other => {
                    return Err(format!(
                        "str.replace: from must be str, got {}",
                        other.type_name()
                    ))
                }
            };
            let to = match to {
                Value::Str(to) => to.as_str(),
                other => {
                    return Err(format!(
                        "str.replace: to must be str, got {}",
                        other.type_name()
                    ))
                }
            };
            Ok(Value::Str(Rc::new(s.replace(from, to))))
        }
        (Value::Str(s), "split", [sep]) => {
            let sep = match sep {
                Value::Str(sep) => sep.as_str(),
                other => {
                    return Err(format!(
                        "str.split: separator must be str, got {}",
                        other.type_name()
                    ))
                }
            };
            let parts = s
                .split(sep)
                .map(|part| Value::Str(Rc::new(part.to_string())))
                .collect();
            Ok(Value::List(Rc::new(RefCell::new(parts))))
        }
        (Value::Str(s), "lines", []) => {
            let lines = s
                .lines()
                .map(|line| Value::Str(Rc::new(line.to_string())))
                .collect();
            Ok(Value::List(Rc::new(RefCell::new(lines))))
        }
        (Value::Map(m), "len", []) => Ok(Value::Int(m.borrow().len() as i64)),
        (Value::Map(m), "contains", [key]) => {
            let key = match key {
                Value::Str(key) => key.as_str(),
                other => {
                    return Err(format!(
                        "map.contains: key must be str, got {}",
                        other.type_name()
                    ))
                }
            };
            Ok(Value::Bool(m.borrow().contains_key(key)))
        }
        (Value::Map(m), "keys", []) => {
            let keys: Vec<Value> = m
                .borrow()
                .keys()
                .map(|k| Value::Str(Rc::new(k.clone())))
                .collect();
            Ok(Value::List(Rc::new(RefCell::new(keys))))
        }
        (Value::Map(m), "values", []) => {
            let values = m.borrow().values().cloned().collect();
            Ok(Value::List(Rc::new(RefCell::new(values))))
        }
        (Value::Result(r), name, args) => call_result_method(r, name, args),
        (t, n, _) => Err(format!("no method `{}` on {}", n, t.type_name())),
    }
}

/// Built-in methods that every capability supports regardless of kind.
/// `narrow` and `revoke` and `is_revoked` live here; `read`/`write`/`get`/...
/// dispatch through Authority::invoke.
pub(crate) fn call_capability_method(
    cap: &Rc<crate::capability::Capability>,
    name: &str,
    args: &[Value],
    rt: &mut dyn Runtime,
) -> Result<Value, String> {
    match (name, args) {
        ("narrow", [params]) => {
            let child = cap.narrow(params)?;
            Ok(Value::Capability(child))
        }
        ("narrow", _) => Err("capability.narrow expects one map argument".into()),
        ("revoke", []) => {
            cap.revoke();
            Ok(Value::Nil)
        }
        ("is_revoked", []) => Ok(Value::Bool(cap.is_revoked())),
        ("kind", []) => Ok(Value::Str(Rc::new(cap.kind.clone()))),
        _ => {
            // Every authority method returns a TetherScript Result so callers can
            // use `?` or `.is_ok()` uniformly. Native-side `Ok(v)` lifts to
            // `Value::Result(Ok(v))`; native-side `Err(e)` lifts to
            // `Value::Result(Err(e))` — not a runtime error, because the
            // call itself succeeded (the authority reported a failure).
            match cap.invoke(rt, name, args) {
                Ok(v) => Ok(Value::Result(Rc::new(crate::value::ResultValue::Ok(v)))),
                Err(e) => Ok(Value::Result(Rc::new(crate::value::ResultValue::Err(e)))),
            }
        }
    }
}

fn call_result_method(
    r: &Rc<crate::value::ResultValue>,
    name: &str,
    args: &[Value],
) -> Result<Value, String> {
    use crate::value::ResultValue;
    match (r.as_ref(), name, args) {
        (ResultValue::Ok(_), "is_ok", []) => Ok(Value::Bool(true)),
        (ResultValue::Err(_), "is_ok", []) => Ok(Value::Bool(false)),
        (ResultValue::Ok(_), "is_err", []) => Ok(Value::Bool(false)),
        (ResultValue::Err(_), "is_err", []) => Ok(Value::Bool(true)),
        (ResultValue::Ok(v), "unwrap", []) => Ok(v.clone()),
        (ResultValue::Err(e), "unwrap", []) => Err(format!("called unwrap on Err({:?})", e)),
        (ResultValue::Ok(v), "unwrap_or", [_]) => Ok(v.clone()),
        (ResultValue::Err(_), "unwrap_or", [d]) => Ok(d.clone()),
        (ResultValue::Ok(v), "ok", []) => Ok(v.clone()),
        (ResultValue::Err(_), "ok", []) => Ok(Value::Nil),
        (ResultValue::Ok(_), "err", []) => Ok(Value::Nil),
        (ResultValue::Err(e), "err", []) => Ok(Value::Str(Rc::new(e.clone()))),
        (_, n, _) => Err(format!("no method `{}` on result", n)),
    }
}

// ---------- step budget (for sandboxed eval) ----------

thread_local! {
    static STEPS_REMAINING: Cell<Option<u64>> = const { Cell::new(None) };
}

/// Run `f` with an AST-step budget enforced on `Interpreter::eval` calls.
/// Outside this wrapper, the interpreter is uncapped.
pub fn with_step_budget<F, R>(budget: u64, f: F) -> R
where
    F: FnOnce() -> R,
{
    let prev = STEPS_REMAINING.with(|s| s.replace(Some(budget)));
    let r = f();
    STEPS_REMAINING.with(|s| s.set(prev));
    r
}

fn step_tick() -> Result<(), Unwind> {
    let exhausted = STEPS_REMAINING.with(|s| match s.get() {
        None => false,
        Some(0) => true,
        Some(n) => {
            s.set(Some(n - 1));
            false
        }
    });
    if exhausted {
        Err(Unwind::Error(
            "eval step budget exhausted (possible infinite loop)".into(),
        ))
    } else {
        Ok(())
    }
}

// ---------- Runtime bridge ----------

/// Lets runtime-aware natives re-enter the tree-walker to invoke a callable.
struct InterpRuntime<'a> {
    interp: &'a Interpreter,
}

impl<'a> Runtime for InterpRuntime<'a> {
    fn invoke(&mut self, callee: &Value, args: &[Value]) -> Result<Value, String> {
        match self.interp.call(callee, args) {
            Ok(v) => Ok(v),
            Err(Unwind::Error(e)) | Err(Unwind::Panic(e)) | Err(Unwind::TryErr(e)) => Err(e),
            Err(Unwind::Return(_)) => Err("`return` unwound out of callback".into()),
        }
    }
}

fn browser_assert(
    rt: &mut dyn Runtime,
    args: &[Value],
    method: &str,
    message: &str,
) -> Result<Value, String> {
    let browser = args
        .first()
        .ok_or_else(|| format!("{}: missing browser", method))?;
    let arg = args.get(1).cloned().unwrap_or(Value::Nil);
    let call_args = if matches!(arg, Value::Nil) {
        Vec::new()
    } else {
        vec![arg]
    };
    match invoke_browser_result(rt, browser, method, &call_args)? {
        crate::value::ResultValue::Ok(_) => Ok(Value::Result(Rc::new(
            crate::value::ResultValue::Ok(Value::Bool(true)),
        ))),
        crate::value::ResultValue::Err(e) => Ok(Value::Result(Rc::new(
            crate::value::ResultValue::Err(format!("{}: {}", message, e)),
        ))),
    }
}

fn browser_assert_empty(
    rt: &mut dyn Runtime,
    args: &[Value],
    method: &str,
    message: &str,
) -> Result<Value, String> {
    let browser = args
        .first()
        .ok_or_else(|| format!("{}: missing browser", method))?;
    match invoke_browser_result(rt, browser, method, &[])? {
        crate::value::ResultValue::Ok(Value::List(xs)) if xs.borrow().is_empty() => Ok(
            Value::Result(Rc::new(crate::value::ResultValue::Ok(Value::Bool(true)))),
        ),
        crate::value::ResultValue::Ok(Value::Nil) => Ok(Value::Result(Rc::new(
            crate::value::ResultValue::Ok(Value::Bool(true)),
        ))),
        crate::value::ResultValue::Ok(v) => Ok(Value::Result(Rc::new(
            crate::value::ResultValue::Err(format!("{}: {}", message, v)),
        ))),
        crate::value::ResultValue::Err(e) => {
            Ok(Value::Result(Rc::new(crate::value::ResultValue::Err(e))))
        }
    }
}

fn browser_assert_bool(
    rt: &mut dyn Runtime,
    args: &[Value],
    method: &str,
    message: &str,
) -> Result<Value, String> {
    let browser = args
        .first()
        .ok_or_else(|| format!("{}: missing browser", method))?;
    let arg = args
        .get(1)
        .cloned()
        .ok_or_else(|| format!("{}: missing selector", method))?;
    match invoke_browser_result(rt, browser, method, &[arg])? {
        crate::value::ResultValue::Ok(Value::Bool(true)) => Ok(Value::Result(Rc::new(
            crate::value::ResultValue::Ok(Value::Bool(true)),
        ))),
        crate::value::ResultValue::Ok(v) => Ok(Value::Result(Rc::new(
            crate::value::ResultValue::Err(format!("{}: {}", message, v)),
        ))),
        crate::value::ResultValue::Err(e) => {
            Ok(Value::Result(Rc::new(crate::value::ResultValue::Err(e))))
        }
    }
}

fn browser_assert_snapshot_flag(
    rt: &mut dyn Runtime,
    args: &[Value],
    _flag: &str,
    message: &str,
) -> Result<Value, String> {
    browser_assert_bool(rt, args, "is_enabled", message)
}

fn invoke_browser_result(
    rt: &mut dyn Runtime,
    browser: &Value,
    method: &str,
    args: &[Value],
) -> Result<crate::value::ResultValue, String> {
    let cap = match browser {
        Value::Capability(c) => c,
        other => {
            return Err(format!(
                "browser assertion expected capability, got {}",
                other.type_name()
            ))
        }
    };
    match cap.invoke(rt, method, args) {
        Ok(v) => Ok(crate::value::ResultValue::Ok(v)),
        Err(e) => Ok(crate::value::ResultValue::Err(e)),
    }
}

// ---------- built-ins ----------

pub(crate) fn install_builtins(env: &Rc<RefCell<Env>>) {
    install_pure_builtins(env);
    install_browser_builtins(env);
    let static_globals = env.clone();
    let mut e = env.borrow_mut();

    e.define(
        "http_serve",
        runtime_native("http_serve", Some(2), |rt, args| {
            http::serve(rt, &args[0], &args[1])
        }),
        false,
    );
    e.define(
        "http_serve_static",
        runtime_native("http_serve_static", Some(2), move |rt, args| {
            http::serve_static(rt, &static_globals, &args[0], &args[1])
        }),
        false,
    );

    e.define(
        "assert_selector",
        runtime_native("assert_selector", Some(2), |rt, args| {
            browser_assert(rt, args, "wait_for_selector", "selector not found")
        }),
        false,
    );
    e.define(
        "assert_text",
        runtime_native("assert_text", Some(2), |rt, args| {
            browser_assert(rt, args, "wait_for_text", "text not found")
        }),
        false,
    );
    e.define(
        "assert_no_console_errors",
        runtime_native("assert_no_console_errors", Some(1), |rt, args| {
            browser_assert_empty(rt, args, "console_errors", "console errors present")
        }),
        false,
    );
    e.define(
        "assert_no_failed_requests",
        runtime_native("assert_no_failed_requests", Some(1), |rt, args| {
            browser_assert_empty(rt, args, "failed_requests", "failed requests present")
        }),
        false,
    );
    e.define(
        "assert_visible",
        runtime_native("assert_visible", Some(2), |rt, args| {
            browser_assert_bool(rt, args, "is_visible", "element is not visible")
        }),
        false,
    );
    e.define(
        "assert_enabled",
        runtime_native("assert_enabled", Some(2), |rt, args| {
            browser_assert_snapshot_flag(rt, args, "enabled", "element is not enabled")
        }),
        false,
    );
    e.define(
        "assert_route",
        runtime_native("assert_route", Some(2), |rt, args| {
            browser_assert(rt, args, "wait_for_url", "route not reached")
        }),
        false,
    );
    e.define(
        "assert_screenshot_matches",
        runtime_native("assert_screenshot_matches", Some(2), |rt, args| {
            browser_assert(rt, args, "assert_screenshot_matches", "screenshot mismatch")
        }),
        false,
    );
    e.define(
        "assert_react_component",
        runtime_native("assert_react_component", Some(2), |rt, args| {
            browser_assert(
                rt,
                args,
                "react.component_tree",
                "react component not found",
            )
        }),
        false,
    );

    e.define(
        "http_get",
        pure_native("http_get", Some(1), |args| Ok(http::get(&args[0]))),
        false,
    );
    e.define(
        "http_head",
        pure_native("http_head", Some(1), |args| Ok(http::head(&args[0]))),
        false,
    );
    e.define(
        "http_post",
        pure_native("http_post", Some(2), |args| {
            Ok(http::post(&args[0], &args[1]))
        }),
        false,
    );
    e.define(
        "http_request",
        pure_native("http_request", None, |args| Ok(http::request(args))),
        false,
    );

    e.define(
        "time_now_ms",
        pure_native("time_now_ms", Some(0), |_args| Ok(system::time_now_ms())),
        false,
    );
    e.define(
        "sleep_ms",
        pure_native("sleep_ms", Some(1), |args| Ok(system::sleep_ms(&args[0]))),
        false,
    );
    e.define(
        "env_get",
        pure_native("env_get", Some(1), |args| Ok(system::env_get(&args[0]))),
        false,
    );
    e.define(
        "cwd",
        pure_native("cwd", Some(0), |_args| Ok(system::cwd())),
        false,
    );
    e.define(
        "chdir",
        pure_native("chdir", Some(1), |args| Ok(system::chdir(&args[0]))),
        false,
    );
    e.define(
        "fs_read",
        pure_native("fs_read", Some(1), |args| Ok(system::fs_read(&args[0]))),
        false,
    );
    e.define(
        "fs_write",
        pure_native("fs_write", Some(2), |args| {
            Ok(system::fs_write(&args[0], &args[1]))
        }),
        false,
    );
    e.define(
        "fs_exists",
        pure_native("fs_exists", Some(1), |args| Ok(system::fs_exists(&args[0]))),
        false,
    );
    e.define(
        "fs_list",
        pure_native("fs_list", Some(1), |args| Ok(system::fs_list(&args[0]))),
        false,
    );
    e.define(
        "fs_mkdir",
        pure_native("fs_mkdir", Some(1), |args| Ok(system::fs_mkdir(&args[0]))),
        false,
    );
    e.define(
        "process_run",
        pure_native("process_run", None, |args| Ok(system::process_run(args))),
        false,
    );
    e.define(
        "process_args",
        pure_native("process_args", Some(0), |_args| Ok(system::process_args())),
        false,
    );
    e.define(
        "process_pid",
        pure_native("process_pid", Some(0), |_args| Ok(system::process_pid())),
        false,
    );
    e.define(
        "process_platform",
        pure_native("process_platform", Some(0), |_args| {
            Ok(system::process_platform())
        }),
        false,
    );
    e.define(
        "process_arch",
        pure_native("process_arch", Some(0), |_args| Ok(system::process_arch())),
        false,
    );
    e.define(
        "os_platform",
        pure_native("os_platform", Some(0), |_args| Ok(system::os_platform())),
        false,
    );
    e.define(
        "os_arch",
        pure_native("os_arch", Some(0), |_args| Ok(system::os_arch())),
        false,
    );
    e.define(
        "os_tmpdir",
        pure_native("os_tmpdir", Some(0), |_args| Ok(system::os_tmpdir())),
        false,
    );
    e.define(
        "os_homedir",
        pure_native("os_homedir", Some(0), |_args| Ok(system::os_homedir())),
        false,
    );
    e.define(
        "os_eol",
        pure_native("os_eol", Some(0), |_args| Ok(system::os_eol())),
        false,
    );
    e.define(
        "fs_stat",
        pure_native("fs_stat", Some(1), |args| Ok(system::fs_stat(&args[0]))),
        false,
    );
    e.define(
        "fs_remove",
        pure_native("fs_remove", Some(1), |args| Ok(system::fs_remove(&args[0]))),
        false,
    );
    e.define(
        "fs_rename",
        pure_native("fs_rename", Some(2), |args| {
            Ok(system::fs_rename(&args[0], &args[1]))
        }),
        false,
    );
    e.define(
        "fs_copy",
        pure_native("fs_copy", Some(2), |args| {
            Ok(system::fs_copy(&args[0], &args[1]))
        }),
        false,
    );
    e.define(
        "path_sep",
        pure_native("path_sep", Some(0), |_args| Ok(system::path_sep())),
        false,
    );
    e.define(
        "path_join",
        pure_native("path_join", Some(1), |args| Ok(system::path_join(&args[0]))),
        false,
    );
    e.define(
        "path_dirname",
        pure_native("path_dirname", Some(1), |args| {
            Ok(system::path_dirname(&args[0]))
        }),
        false,
    );
    e.define(
        "path_basename",
        pure_native("path_basename", Some(1), |args| {
            Ok(system::path_basename(&args[0]))
        }),
        false,
    );
    e.define(
        "path_extname",
        pure_native("path_extname", Some(1), |args| {
            Ok(system::path_extname(&args[0]))
        }),
        false,
    );
    e.define(
        "path_normalize",
        pure_native("path_normalize", Some(1), |args| {
            Ok(system::path_normalize(&args[0]))
        }),
        false,
    );
    e.define(
        "path_resolve",
        pure_native("path_resolve", Some(1), |args| {
            Ok(system::path_resolve(&args[0]))
        }),
        false,
    );
    e.define(
        "url_parse",
        pure_native("url_parse", Some(1), |args| Ok(system::url_parse(&args[0]))),
        false,
    );
    e.define(
        "sha256_hex",
        pure_native("sha256_hex", Some(1), |args| {
            Ok(system::sha256_hex(&args[0]))
        }),
        false,
    );
    e.define(
        "base64_encode",
        pure_native("base64_encode", Some(1), |args| {
            Ok(system::base64_encode(&args[0]))
        }),
        false,
    );
    e.define(
        "base64_decode",
        pure_native("base64_decode", Some(1), |args| {
            Ok(system::base64_decode(&args[0]))
        }),
        false,
    );

    e.define(
        "smtp_send",
        pure_native("smtp_send", Some(6), |args| {
            smtp::send(&args[0], &args[1], &args[2], &args[3], &args[4], &args[5])
        }),
        false,
    );

    e.define(
        "json_parse",
        pure_native("json_parse", Some(1), |args| json::parse(&args[0])),
        false,
    );
    e.define(
        "json_encode",
        pure_native("json_encode", Some(1), |args| json::encode(&args[0])),
        false,
    );
    e.define(
        "json_encode_pretty",
        pure_native("json_encode_pretty", Some(1), |args| {
            json::encode_pretty(&args[0])
        }),
        false,
    );

    e.define(
        "eval",
        pure_native("eval", Some(1), |args| eval_source(&args[0])),
        false,
    );
    e.define(
        "js_eval",
        pure_native("js_eval", Some(1), js::eval_to_value),
        false,
    );
}

/// Subset of built-ins safe to expose to untrusted source running inside
/// `eval()`: no network, no nested `eval`. Same `println`/`print`/`len`/
/// `type_of`/`map` set every other example uses.
fn install_sandbox_builtins(env: &Rc<RefCell<Env>>) {
    install_pure_builtins(env);
}

fn install_browser_builtins(env: &Rc<RefCell<Env>>) {
    let mut e = env.borrow_mut();

    e.define(
        "browser_parse_html",
        pure_native("browser_parse_html", Some(1), browser::html_to_value),
        false,
    );
    e.define(
        "browser_parse_css",
        pure_native("browser_parse_css", Some(1), browser::css_to_value),
        false,
    );
    e.define(
        "browser_styles",
        pure_native("browser_styles", None, browser::styles_to_value),
        false,
    );
    e.define(
        "browser_query_selector",
        pure_native(
            "browser_query_selector",
            Some(2),
            browser::query_selector_to_value,
        ),
        false,
    );
    e.define(
        "browser_text_content",
        pure_native(
            "browser_text_content",
            Some(2),
            browser::text_content_to_value,
        ),
        false,
    );
    e.define(
        "browser_snapshot",
        pure_native("browser_snapshot", None, browser::snapshot_to_value),
        false,
    );
    e.define(
        "browser_display_list",
        pure_native(
            "browser_display_list",
            None,
            browser::display_list_to_runtime_value,
        ),
        false,
    );
    e.define(
        "browser_render",
        pure_native("browser_render", None, browser::render_to_value),
        false,
    );
    e.define(
        "browser_raster",
        pure_native("browser_raster", None, browser::raster_to_runtime_value),
        false,
    );
    e.define(
        "browser_render_ppm",
        pure_native("browser_render_ppm", None, browser::render_ppm_to_value),
        false,
    );
    e.define(
        "browser_layout",
        pure_native("browser_layout", None, browser::layout_to_runtime_value),
        false,
    );
    e.define(
        "browser_run_scripts",
        pure_native(
            "browser_run_scripts",
            Some(1),
            browser_js::browser_run_scripts_to_value,
        ),
        false,
    );
    e.define(
        "browser_run_scripts_at",
        pure_native(
            "browser_run_scripts_at",
            Some(2),
            browser_js::browser_run_scripts_at_to_value,
        ),
        false,
    );
    e.define(
        "browser_eval_js",
        pure_native(
            "browser_eval_js",
            Some(2),
            browser_js::browser_eval_js_to_value,
        ),
        false,
    );
    e.define(
        "browser_compatibility_report",
        pure_native(
            "browser_compatibility_report",
            Some(0),
            browser_js::compatibility_report_to_value,
        ),
        false,
    );
}

fn install_pure_builtins(env: &Rc<RefCell<Env>>) {
    let mut e = env.borrow_mut();

    e.define(
        "println",
        pure_native("println", None, |args| {
            let s: Vec<String> = args.iter().map(|v| v.to_string()).collect();
            output::writeln(&s.join(" "));
            Ok(Value::Nil)
        }),
        false,
    );

    e.define(
        "print",
        pure_native("print", None, |args| {
            let s: Vec<String> = args.iter().map(|v| v.to_string()).collect();
            output::write(&s.join(" "));
            Ok(Value::Nil)
        }),
        false,
    );

    e.define(
        "len",
        pure_native("len", Some(1), |args| match &args[0] {
            Value::Str(s) => Ok(Value::Int(s.len() as i64)),
            Value::Bytes(b) => Ok(Value::Int(b.borrow().len() as i64)),
            Value::List(x) => Ok(Value::Int(x.borrow().len() as i64)),
            Value::Map(m) => Ok(Value::Int(m.borrow().len() as i64)),
            v => Err(format!("len: {} has no length", v.type_name())),
        }),
        false,
    );

    e.define(
        "type_of",
        pure_native("type_of", Some(1), |args| {
            Ok(Value::Str(Rc::new(args[0].type_name().into())))
        }),
        false,
    );

    e.define(
        "map",
        pure_native("map", Some(0), |_args| {
            Ok(Value::Map(Rc::new(RefCell::new(HashMap::new()))))
        }),
        false,
    );

    e.define(
        "str",
        pure_native("str", Some(1), |args| Ok(system::value_to_string(&args[0]))),
        false,
    );
    e.define(
        "bytes",
        pure_native("bytes", Some(1), |args| match &args[0] {
            Value::Str(s) => Ok(Value::Bytes(Rc::new(RefCell::new(s.as_bytes().to_vec())))),
            Value::List(items) => {
                let mut out = Vec::with_capacity(items.borrow().len());
                for item in items.borrow().iter() {
                    match item {
                        Value::Int(n) => out
                            .push(u8::try_from(*n).map_err(|_| {
                                "bytes: list values must be in 0..=255".to_string()
                            })?),
                        other => {
                            return Err(format!(
                                "bytes: list values must be int, got {}",
                                other.type_name()
                            ))
                        }
                    }
                }
                Ok(Value::Bytes(Rc::new(RefCell::new(out))))
            }
            Value::Bytes(b) => Ok(Value::Bytes(Rc::new(RefCell::new(b.borrow().clone())))),
            other => Err(format!(
                "bytes: expected str, list, or bytes; got {}",
                other.type_name()
            )),
        }),
        false,
    );
    e.define(
        "parse_int",
        pure_native("parse_int", Some(1), |args| Ok(system::parse_int(&args[0]))),
        false,
    );
    e.define(
        "parse_float",
        pure_native("parse_float", Some(1), |args| {
            Ok(system::parse_float(&args[0]))
        }),
        false,
    );
    e.define(
        "assert",
        pure_native("assert", None, system::assert_builtin),
        false,
    );

    e.define(
        "Ok",
        pure_native("Ok", Some(1), |args| {
            Ok(Value::Result(Rc::new(crate::value::ResultValue::Ok(
                args[0].clone(),
            ))))
        }),
        false,
    );

    e.define(
        "Err",
        pure_native("Err", Some(1), |args| {
            let msg = match &args[0] {
                Value::Str(s) => (**s).clone(),
                other => other.to_string(),
            };
            Ok(Value::Result(Rc::new(crate::value::ResultValue::Err(msg))))
        }),
        false,
    );
}

const EVAL_STEP_BUDGET: u64 = 200_000;
const EVAL_OUTPUT_LIMIT: usize = 64 * 1024;

fn eval_source(arg: &Value) -> Result<Value, String> {
    let src = match arg {
        Value::Str(s) => (**s).clone(),
        other => {
            return Err(format!(
                "eval: source must be a string, got {}",
                other.type_name()
            ))
        }
    };
    let (captured, result) = output::with_capture(EVAL_OUTPUT_LIMIT, || {
        with_step_budget(EVAL_STEP_BUDGET, || run_sandboxed(&src))
    });
    Ok(Value::Str(Rc::new(format_eval_output(captured, result))))
}

fn run_sandboxed(src: &str) -> Result<Value, String> {
    let tokens = Lexer::new(src)
        .tokenize()
        .map_err(|e| format!("lex error at {}:{}: {}", e.line, e.col, e.msg))?;
    let program = Parser::new(tokens)
        .parse_program()
        .map_err(|e| format!("parse error at {}:{}: {}", e.line, e.col, e.msg))?;
    let mut interp = Interpreter::new_sandboxed();
    interp.run_repl(&program)
}

fn format_eval_output(mut out: String, result: Result<Value, String>) -> String {
    match result {
        Ok(Value::Nil) => {
            if out.is_empty() {
                out.push_str("=> nil\n");
            }
        }
        Ok(v) => {
            if !out.is_empty() && !out.ends_with('\n') {
                out.push('\n');
            }
            out.push_str("=> ");
            out.push_str(&v.to_string());
            out.push('\n');
        }
        Err(e) => {
            if !out.is_empty() && !out.ends_with('\n') {
                out.push('\n');
            }
            out.push_str("error: ");
            out.push_str(&e);
            out.push('\n');
        }
    }
    out
}

fn pure_native<F>(name: &str, arity: Option<usize>, func: F) -> Value
where
    F: Fn(&[Value]) -> Result<Value, String> + 'static,
{
    Value::Native(Rc::new(NativeFn {
        name: name.to_string(),
        arity,
        func: NativeFunc::Pure(Box::new(func)),
    }))
}

fn runtime_native<F>(name: &str, arity: Option<usize>, func: F) -> Value
where
    F: Fn(&mut dyn Runtime, &[Value]) -> Result<Value, String> + 'static,
{
    Value::Native(Rc::new(NativeFn {
        name: name.to_string(),
        arity,
        func: NativeFunc::Runtime(Box::new(func)),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::Compiler;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::vm::VM;

    fn parse(source: &str) -> Program {
        let tokens = Lexer::new(source).tokenize().unwrap();
        Parser::new(tokens).parse_program().unwrap()
    }

    #[test]
    fn standard_tools_work_in_interpreter() {
        let source = r#"
fn main() {
    let parts = " alpha,beta ".trim().split(",")
    assert(parts.join("|") == "alpha|beta", "split/join failed")
    assert(parts.contains("alpha"), "list contains failed")
    assert("tetherscript".starts_with("tether"), "starts_with failed")
    assert("tetherscript".ends_with("script"), "ends_with failed")
    assert("tetherscript".replace("script", "") == "tether", "replace failed")

    let m = map()
    m.answer = 42
    assert(m.contains("answer"), "map contains failed")
    assert(m.values().contains(42), "map values failed")

    assert(str(42) == "42", "str failed")
    assert(parse_int("42").unwrap() == 42, "parse_int failed")
    assert(parse_float("2.5").unwrap() == 2.5, "parse_float failed")
    assert(time_now_ms() > 0, "time failed")
    assert(process_pid() > 0, "pid failed")
    assert(process_platform().len() > 0, "platform failed")
    assert(os_arch().len() > 0, "arch failed")
    assert(path_basename("alpha/beta.txt").unwrap() == "beta.txt", "basename failed")
    assert(path_extname("alpha/beta.txt").unwrap() == ".txt", "extname failed")
    assert(sha256_hex("tetherscript").unwrap() == "a724f07d8f90ed2c1c123a60fa8d8118f95f96dc4de19121bf91306a6bdbdb55", "sha failed")
    assert(base64_decode(base64_encode("tetherscript").unwrap()).unwrap() == "tetherscript", "base64 failed")
    assert(url_parse("http://example.com/a?b=c").unwrap().host == "example.com", "url failed")
}
"#;
        let program = parse(source);
        let mut interp = Interpreter::new();
        interp.run(&program).unwrap();
    }

    #[test]
    fn standard_tools_work_in_vm() {
        let source = r#"
fn main() {
    let parts = " alpha,beta ".trim().split(",")
    assert(parts.join("|") == "alpha|beta", "split/join failed")
    assert(parts.contains("beta"), "list contains failed")
    assert("tetherscript".contains("script"), "contains failed")

    let m = map()
    m.answer = 42
    assert(m.keys().contains("answer"), "map keys failed")

    assert(str(42) == "42", "str failed")
    assert(parse_int("42").unwrap() == 42, "parse_int failed")
    assert(time_now_ms() > 0, "time failed")
    assert(path_basename("alpha/beta.txt").unwrap() == "beta.txt", "basename failed")
    assert(base64_decode(base64_encode("tetherscript").unwrap()).unwrap() == "tetherscript", "base64 failed")
}
"#;
        let program = parse(source);
        let chunk = Compiler::compile_program(&program);
        let mut vm = VM::new();
        vm.run(chunk).unwrap();
    }

    #[test]
    fn bytes_literals_and_methods_match_interpreter_and_vm() {
        let source = r#"
fn make() {
    let b = b"hi\x21"
    b[0] = 72
    b.push(10)
    assert(b.len() == 4, "bytes len failed")
    assert(b[0] == 72, "bytes index assignment failed")
    assert(b[1] == 105, "bytes index failed")
    assert(b.pop() == 10, "bytes pop failed")
    assert(b.hex() == "486921", "bytes hex failed")
    assert(b.decode_utf8() == "Hi!", "bytes decode failed")
    return b
}

fn main() {
    let a = make()
    let b = make()
    assert(a.hex() == "486921", "first literal result failed")
    assert(b.hex() == "486921", "second literal result failed")
    assert(bytes([111, 107]).decode_utf8() == "ok", "bytes(list) failed")
}
"#;
        let program = parse(source);

        let mut interp = Interpreter::new();
        interp.run(&program).unwrap();

        let chunk = Compiler::compile_program(&program);
        let mut vm = VM::new();
        vm.run(chunk).unwrap();
    }

    #[test]
    fn for_loops_work_in_interpreter_and_vm() {
        let source = r#"
fn main() {
    let nums = [1, 2, 3, 4]
    let mut sum = 0
    for n in nums {
        sum = sum + n
    }
    assert(sum == 10, "list for loop failed")

    let mut text = ""
    for ch in "ts" {
        text = text + ch
    }
    assert(text == "ts", "string for loop failed")

    let m = map()
    m.beta = 2
    m.alpha = 1
    let keys = []
    for key in m {
        keys.push(key)
    }
    assert(keys.join(",") == "alpha,beta", "map key for loop failed")

    let growing = [1, 2]
    let seen = []
    for value in growing {
        seen.push(value)
        growing.push(99)
    }
    assert(seen.join(",") == "1,2", "for loop should snapshot iterable")
}
"#;
        let program = parse(source);

        let mut interp = Interpreter::new();
        interp.run(&program).unwrap();

        let chunk = Compiler::compile_program(&program);
        let mut vm = VM::new();
        vm.run(chunk).unwrap();
    }
}
