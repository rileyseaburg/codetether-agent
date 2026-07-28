//! Access to the granted filesystem capability used for preload.

use std::cell::RefCell;
use std::rc::Rc;

use crate::capability::Capability;
use crate::value::{Env, Value};

/// Fetch the global `fs` capability installed by `--grant-fs`.
pub(crate) fn capability(globals: &Rc<RefCell<Env>>) -> Result<Rc<Capability>, String> {
    match globals.borrow().get("fs") {
        Ok(Value::Capability(capability)) => Ok(capability),
        Ok(other) => Err(format!(
            "http_serve_static: global fs must be a capability, got {}",
            other.type_name()
        )),
        Err(_) => Err(
            "http_serve_static: filesystem access requires `tetherscript run --grant-fs <dir>`"
                .into(),
        ),
    }
}
