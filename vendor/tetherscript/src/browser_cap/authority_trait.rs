//! Authority trait implementation for browser authorities.

use std::any::Any;
use std::rc::Rc;

use crate::capability::Authority;
use crate::value::{Runtime, Value};

use super::BrowserAuthority;

impl Authority for BrowserAuthority {
    fn narrow(&self, params: &Value) -> Result<Rc<dyn Authority>, String> {
        self.narrowed(params)
    }

    fn invoke(&self, _rt: &mut dyn Runtime, method: &str, args: &[Value]) -> Result<Value, String> {
        self.invoke_method(method, args)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
