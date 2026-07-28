//! Invocation flow for browser authority methods.

use crate::{json, value::Value};

use super::authority::BrowserAuthority;
use super::invoke_helpers::remote_or;

impl BrowserAuthority {
    pub(crate) fn invoke_method(&self, method: &str, args: &[Value]) -> Result<Value, String> {
        match method {
            "describe" => Ok(self.describe()),
            "trace" | "export_trace_json" => Ok(self.trace.borrow().value()),
            "export_har" => remote_or(method, self.call_checked(method, args), || {
                self.trace.borrow().har()
            }),
            "agent_summary" => remote_or(method, self.call_checked(method, args), || {
                self.trace.borrow().summary()
            }),
            "minimal_reproduction_script" => {
                remote_or(method, self.call_checked(method, args), || {
                    self.minimal_repro()
                })
            }
            _ => self.call_checked(method, args),
        }
    }

    fn call_checked(&self, method: &str, args: &[Value]) -> Result<Value, String> {
        let call = super::mapping::prepare(self, method, args)?;
        self.require_scope(call.scope)?;
        self.approval_check(method, &call.action)?;
        self.trace.borrow_mut().action(method, call.payload.clone());
        match self.invoke_remote(method, &call.payload) {
            Ok(value) => {
                self.trace.borrow_mut().observe(method, &value);
                Ok(value)
            }
            Err(e) => {
                self.trace.borrow_mut().error(method, e.clone());
                Err(e)
            }
        }
    }

    fn invoke_remote(&self, method: &str, payload: &Value) -> Result<Value, String> {
        let body = json::encode_to_string(payload)?;
        let response = super::transport::post_json(&self.endpoint, &body, self.timeout)?;
        if response.trim().is_empty() {
            return Ok(Value::Nil);
        }
        let parsed = json::parse_str(&response)
            .map_err(|e| format!("browser.{}: invalid bridge JSON: {}", method, e))?;
        super::response::normalize(method, parsed)
    }
}
