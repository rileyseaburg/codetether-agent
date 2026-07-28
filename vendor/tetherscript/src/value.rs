//! Runtime values.
//!
//! Every value in TetherScript is a `Value`. Heap-backed values (strings, lists, maps,
//! functions) carry their payload behind an `Rc` so the interpreter can
//! clone Values cheaply.
//!
//! Ownership model:
//! - Scalars (Int, Float, Bool, Nil) are `Copy`. "Moving" one clones it.
//! - Heap values are genuinely owned. When bound to a variable, the variable
//!   slot holds the sole ownership. Passing the binding to a function without
//!   `move` creates a *borrow* — an alias that must not outlive the binding.
//!   Passing with `move` transfers the Value out of the slot and leaves a
//!   `Moved` tombstone behind.
//! - Using a `Moved` binding is a runtime panic.
//!
//! Runtime borrow checking (v0): we only enforce the "no use after move"
//! invariant. Aliasing-xor-mutability via `&mut` is a TODO — today `&mut`
//! parses but behaves like an implicit mutable alias.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::ast::Block;
use crate::bytecode::VmFnObj;

/// A TetherScript value. Heap-backed payloads are Rc'd so cloning is cheap and
/// aliasing is shared by default.
#[derive(Clone)]
pub enum Value {
    Nil,
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(Rc<String>),
    Bytes(Rc<RefCell<Vec<u8>>>),
    List(Rc<RefCell<Vec<Value>>>),
    Map(Rc<RefCell<HashMap<String, Value>>>),
    Fn(Rc<FnObj>),
    VmFn(Rc<VmFnObj>),
    Native(Rc<NativeFn>),
    /// Result<T, str> value. v0.1 error type is always a string; full
    /// generic errors are future work.
    Result(Rc<ResultValue>),
    /// First-class capability (authority grant). See src/capability.rs.
    Capability(Rc<crate::capability::Capability>),
}

/// `Ok(v)` or `Err(message)`. Held by `Value::Result`.
#[derive(Clone)]
pub enum ResultValue {
    Ok(Value),
    Err(String),
}

pub struct FnObj {
    pub params: Vec<String>,
    pub body: Rc<Block>,
    /// Captured environment (closure).
    pub closure: Rc<RefCell<Env>>,
    pub name: Option<String>,
}

pub struct NativeFn {
    pub name: String,
    pub arity: Option<usize>, // None = variadic
    pub func: NativeFunc,
}

pub type PureNativeFn = dyn Fn(&[Value]) -> Result<Value, String>;
pub type RuntimeNativeFn = dyn Fn(&mut dyn Runtime, &[Value]) -> Result<Value, String>;

/// Native built-ins come in two flavors.
///
/// `Pure` natives only read their argument values — they can't call back into
/// the runtime. This is the common case (println, len, type_of, …).
///
/// `Runtime` natives receive a `&mut dyn Runtime` and can invoke any callable
/// Value synchronously. That's how things like `http_serve(port, handler)`
/// dispatch incoming requests back to user-defined TetherScript functions without
/// the native layer having to know whether the caller is the tree-walker or
/// the bytecode VM.
pub enum NativeFunc {
    Pure(Box<PureNativeFn>),
    Runtime(Box<RuntimeNativeFn>),
}

/// Abstraction over the active execution engine. Both the tree-walking
/// interpreter and the bytecode VM implement this so runtime-aware natives
/// (see `NativeFunc::Runtime`) can invoke TetherScript callables uniformly.
pub trait Runtime {
    /// Synchronously call `callee` with `args` and return its result.
    /// Errors and panics bubble up as `Err(String)`.
    fn invoke(&mut self, callee: &Value, args: &[Value]) -> Result<Value, String>;
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Nil => "nil",
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::Bool(_) => "bool",
            Value::Str(_) => "str",
            Value::Bytes(_) => "bytes",
            Value::List(_) => "list",
            Value::Map(_) => "map",
            Value::Fn(_) | Value::VmFn(_) | Value::Native(_) => "fn",
            Value::Result(_) => "result",
            Value::Capability(_) => "capability",
        }
    }

    pub fn truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => *b,
            Value::Int(n) => *n != 0,
            Value::Float(f) => *f != 0.0,
            Value::Str(s) => !s.is_empty(),
            Value::Bytes(b) => !b.borrow().is_empty(),
            Value::List(xs) => !xs.borrow().is_empty(),
            Value::Map(m) => !m.borrow().is_empty(),
            Value::Fn(_) | Value::VmFn(_) | Value::Native(_) => true,
            Value::Result(r) => matches!(r.as_ref(), ResultValue::Ok(_)),
            Value::Capability(c) => !c.is_revoked(),
        }
    }

    /// Scalars are Copy; heap values are not.
    pub fn is_copy(&self) -> bool {
        matches!(
            self,
            Value::Nil | Value::Int(_) | Value::Float(_) | Value::Bool(_)
        )
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (Nil, Nil) => true,
            (Int(a), Int(b)) => a == b,
            (Float(a), Float(b)) => a == b,
            (Bool(a), Bool(b)) => a == b,
            (Str(a), Str(b)) => a == b,
            (Bytes(a), Bytes(b)) => *a.borrow() == *b.borrow(),
            (List(a), List(b)) => *a.borrow() == *b.borrow(),
            (Map(a), Map(b)) => *a.borrow() == *b.borrow(),
            (Result(a), Result(b)) => a.as_ref() == b.as_ref(),
            (Fn(a), Fn(b)) => Rc::ptr_eq(a, b),
            (VmFn(a), VmFn(b)) => Rc::ptr_eq(a, b),
            (Native(a), Native(b)) => Rc::ptr_eq(a, b),
            (Capability(a), Capability(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl PartialEq for ResultValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ResultValue::Ok(a), ResultValue::Ok(b)) => a == b,
            (ResultValue::Err(a), ResultValue::Err(b)) => a == b,
            _ => false,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Str(s) => write!(f, "{}", s),
            Value::Bytes(bytes) => {
                let bytes = bytes.borrow();
                write!(f, "b\"")?;
                for byte in bytes.iter() {
                    match *byte {
                        b'\\' => write!(f, "\\\\")?,
                        b'\"' => write!(f, "\\\"")?,
                        b'\n' => write!(f, "\\n")?,
                        b'\r' => write!(f, "\\r")?,
                        b'\t' => write!(f, "\\t")?,
                        0x20..=0x7e => write!(f, "{}", *byte as char)?,
                        other => write!(f, "\\x{:02x}", other)?,
                    }
                }
                write!(f, "\"")
            }
            Value::List(xs) => {
                let xs = xs.borrow();
                write!(f, "[")?;
                for (i, v) in xs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Map(m) => {
                let m = m.borrow();
                write!(f, "{{")?;
                for (i, (k, v)) in m.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
            Value::Fn(fo) => write!(f, "<fn {}>", fo.name.as_deref().unwrap_or("anon")),
            Value::VmFn(vf) => write!(f, "<fn {}>", vf.name.as_deref().unwrap_or("anon")),
            Value::Native(n) => write!(f, "<native {}>", n.name),
            Value::Result(r) => match r.as_ref() {
                ResultValue::Ok(v) => write!(f, "Ok({})", v),
                ResultValue::Err(e) => write!(f, "Err({:?})", e),
            },
            Value::Capability(c) => {
                if c.is_revoked() {
                    write!(f, "<capability {} (revoked)>", c.kind)
                } else {
                    write!(f, "<capability {}>", c.kind)
                }
            }
        }
    }
}

// ---------- Environments with ownership slots ----------

/// A single binding slot. Holds either a live Value, a tombstone for a moved
/// value, or nothing (uninitialized — shouldn't happen in practice).
#[derive(Clone)]
pub enum Slot {
    Live { value: Value, mutable: bool },
    Moved { name: String }, // keep the name for error messages
}

/// Lexical environment. Parents are `Rc<RefCell<Env>>` so closures can share.
pub struct Env {
    pub slots: HashMap<String, Slot>,
    pub parent: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new_global() -> Rc<RefCell<Env>> {
        Rc::new(RefCell::new(Env {
            slots: HashMap::new(),
            parent: None,
        }))
    }

    pub fn child(parent: &Rc<RefCell<Env>>) -> Rc<RefCell<Env>> {
        Rc::new(RefCell::new(Env {
            slots: HashMap::new(),
            parent: Some(parent.clone()),
        }))
    }

    pub fn define(&mut self, name: &str, value: Value, mutable: bool) {
        self.slots
            .insert(name.to_string(), Slot::Live { value, mutable });
    }

    /// Read (borrow) a binding. For copy values this clones; for heap values
    /// this returns a cheap Rc-clone, *not* a move — the slot stays Live.
    pub fn get(&self, name: &str) -> Result<Value, String> {
        match self.slots.get(name) {
            Some(Slot::Live { value, .. }) => Ok(value.clone()),
            Some(Slot::Moved { name }) => Err(format!(
                "use of moved value `{}` (value was moved earlier and cannot be used again)",
                name
            )),
            None => match &self.parent {
                Some(p) => p.borrow().get(name),
                None => Err(format!("undefined variable `{}`", name)),
            },
        }
    }

    /// Take (move) the Value out of the slot, leaving a tombstone. Used when
    /// the source has an explicit `move` prefix. For scalars this is a clone
    /// (they're Copy) and the slot stays Live.
    pub fn take(&mut self, name: &str) -> Result<Value, String> {
        if let Some(slot) = self.slots.get_mut(name) {
            match slot {
                Slot::Live { value, .. } => {
                    if value.is_copy() {
                        return Ok(value.clone());
                    }
                    let v = value.clone(); // Rc-clone, cheap
                    *slot = Slot::Moved {
                        name: name.to_string(),
                    };
                    Ok(v)
                }
                Slot::Moved { name } => Err(format!(
                    "cannot move from `{}`: value was already moved",
                    name
                )),
            }
        } else {
            match &self.parent {
                Some(p) => p.borrow_mut().take(name),
                None => Err(format!("undefined variable `{}`", name)),
            }
        }
    }

    /// Assign a new value to an existing mutable binding.
    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        if let Some(slot) = self.slots.get_mut(name) {
            match slot {
                Slot::Live { mutable, .. } => {
                    if !*mutable {
                        return Err(format!("cannot assign to immutable binding `{}`", name));
                    }
                    *slot = Slot::Live {
                        value,
                        mutable: true,
                    };
                    Ok(())
                }
                Slot::Moved { name } => Err(format!(
                    "cannot assign to `{}`: slot is a moved tombstone",
                    name
                )),
            }
        } else {
            match &self.parent {
                Some(p) => p.borrow_mut().assign(name, value),
                None => Err(format!("undefined variable `{}`", name)),
            }
        }
    }
}
