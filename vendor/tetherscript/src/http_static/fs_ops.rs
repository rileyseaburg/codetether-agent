//! Filesystem capability operations used while preloading static routes.

use std::rc::Rc;

use crate::capability::Capability;
use crate::value::{Runtime, Value};

/// Call `fs.list` and coerce the returned list into file names.
pub(crate) fn list_dir(
    rt: &mut dyn Runtime,
    fs: &Rc<Capability>,
    path: &str,
) -> Result<Vec<String>, String> {
    match fs.invoke(rt, "list", &[Value::Str(Rc::new(path.to_string()))])? {
        Value::List(items) => items.borrow().iter().map(expect_str).collect(),
        other => Err(format!(
            "fs.list returned {}, expected list",
            other.type_name()
        )),
    }
}

/// Call `fs.read` and return raw bytes.
pub(crate) fn read_file(
    rt: &mut dyn Runtime,
    fs: &Rc<Capability>,
    path: &str,
    list_error: &str,
) -> Result<Vec<u8>, String> {
    match fs.invoke(rt, "read", &[Value::Str(Rc::new(path.to_string()))]) {
        Ok(Value::Str(text)) => Ok(text.as_bytes().to_vec()),
        Ok(Value::Bytes(bytes)) => Ok(bytes.borrow().clone()),
        Ok(other) => Err(format!(
            "fs.read returned {}, expected str/bytes",
            other.type_name()
        )),
        Err(read_error) => Err(format!("{}; {}", list_error, read_error)),
    }
}

fn expect_str(value: &Value) -> Result<String, String> {
    match value {
        Value::Str(text) => Ok((**text).clone()),
        other => Err(format!(
            "fs.list item must be str, got {}",
            other.type_name()
        )),
    }
}
