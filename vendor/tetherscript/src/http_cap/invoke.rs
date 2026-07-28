use std::any::Any;
use std::rc::Rc;

use crate::capability::Authority;
use crate::value::{Runtime, Value};

use super::authority::HttpAuthority;

impl Authority for HttpAuthority {
    fn narrow(&self, params: &Value) -> Result<Rc<dyn Authority>, String> {
        super::narrow::narrow(self, params)
    }

    fn invoke(&self, _rt: &mut dyn Runtime, method: &str, args: &[Value]) -> Result<Value, String> {
        match (method, args) {
            ("get", [Value::Str(url)]) => self.do_request("GET", url, None),
            ("head", [Value::Str(url)]) => self.do_request("HEAD", url, None),
            ("post", [Value::Str(url), Value::Str(body)]) => {
                self.do_request("POST", url, Some(body))
            }
            ("describe", []) => Ok(super::describe::describe(self)),
            (method, _) => Err(format!(
                "http: no method `{}` (have: get, post, head, describe)",
                method
            )),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
