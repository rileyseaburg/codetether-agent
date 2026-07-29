use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use tetherscript::value::{ResultValue, Value};

pub const MAX_BYTES: usize = 1024 * 1024;

pub struct PipeOutput {
    pub text: String,
    pub truncated: bool,
}

/// Unwraps one TetherScript `Result` layer into a host `Result`.
///
/// `tetherscript::system::*` helpers return an already-wrapped
/// `Value::Result`, but the authority contract expects a bare `Value` that the
/// interpreter lifts into exactly one `Result`. Passing the wrapped value
/// straight through would yield `Result<Result<..>>`.
pub fn unwrap_result(value: Value) -> Result<Value, String> {
    match value {
        Value::Result(result) => match &*result {
            ResultValue::Ok(inner) => Ok(inner.clone()),
            ResultValue::Err(error) => Err(error.clone()),
        },
        other => Ok(other),
    }
}

pub fn string(value: impl Into<String>) -> Value {
    Value::Str(Rc::new(value.into()))
}

pub fn map(fields: HashMap<String, Value>) -> Value {
    Value::Map(Rc::new(RefCell::new(fields)))
}
