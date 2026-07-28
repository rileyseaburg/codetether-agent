//! Browserctl action envelope prepared from a tetherscript method call.

use crate::value::Value;

#[derive(Clone, Debug)]
pub(crate) struct BrowserCall {
    pub(crate) action: String,
    pub(crate) payload: Value,
    pub(crate) scope: &'static str,
}

impl BrowserCall {
    pub(crate) fn new(action: &str, scope: &'static str, payload: Value) -> Self {
        Self {
            action: action.into(),
            payload,
            scope,
        }
    }
}
