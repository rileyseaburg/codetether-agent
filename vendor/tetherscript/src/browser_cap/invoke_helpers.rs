//! Small helpers for browser authority invocation.

use crate::value::Value;

use super::authority::BrowserAuthority;

#[path = "invoke_repro.rs"]
mod invoke_repro;

impl BrowserAuthority {
    pub(crate) fn approval_check(&self, method: &str, action: &str) -> Result<(), String> {
        if self.human_approval && (super::actions::is_mutating(action) || mutating_method(method)) {
            Err(format!(
                "browser.{}: human approval gate is enabled; host approval required",
                method
            ))
        } else {
            Ok(())
        }
    }

    pub(crate) fn describe(&self) -> Value {
        super::value::map_value(vec![
            ("endpoint", super::value::str_value(self.endpoint.clone())),
            (
                "origins",
                super::value::list_str(self.allowed_origins.clone()),
            ),
            ("scopes", super::value::list_str(sorted_scopes(self))),
            ("path_prefix", super::value::opt_str(&self.path_prefix)),
            ("storage_scope", super::value::opt_str(&self.storage_scope)),
            ("human_approval", Value::Bool(self.human_approval)),
        ])
    }
}

pub(crate) fn remote_or<F>(
    _: &str,
    result: Result<Value, String>,
    fallback: F,
) -> Result<Value, String>
where
    F: FnOnce() -> Value,
{
    result.or_else(|_| Ok(fallback()))
}

fn sorted_scopes(auth: &BrowserAuthority) -> Vec<String> {
    let mut scopes: Vec<_> = auth.allowed_scopes.iter().cloned().collect();
    scopes.sort();
    scopes
}

fn mutating_method(method: &str) -> bool {
    matches!(method, "set_cookie" | "set_local_storage" | "clear_storage")
}
