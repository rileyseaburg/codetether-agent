use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use tetherscript::value::{ResultValue, Value};

pub const MAX_BYTES: usize = 1024 * 1024;

pub struct PipeOutput {
    pub text: String,
    pub truncated: bool,
}

pub fn wrap(result: Result<Value, String>) -> Value {
    Value::Result(Rc::new(match result {
        Ok(value) => ResultValue::Ok(value),
        Err(error) => ResultValue::Err(error),
    }))
}

pub fn string(value: impl Into<String>) -> Value {
    Value::Str(Rc::new(value.into()))
}

pub fn map(fields: HashMap<String, Value>) -> Value {
    Value::Map(Rc::new(RefCell::new(fields)))
}
