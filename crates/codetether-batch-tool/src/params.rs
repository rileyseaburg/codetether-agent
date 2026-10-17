//! Batch call records and accepted argument aliases.
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub(super) struct Params {
    pub calls: Vec<BatchCall>,
}

#[derive(Deserialize)]
pub(super) struct BatchCall {
    #[serde(alias = "name")]
    pub tool: String,
    #[serde(default, alias = "arguments", alias = "params")]
    pub args: Value,
}
#[cfg(test)]
#[path = "params_tests.rs"]
mod tests;
