//! Typed inputs for the read-only Monarch facade.
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum Input {
    Status {},
    ListTools {},
    Call {
        tool: String,
        #[serde(default = "empty_arguments")]
        arguments: Value,
    },
}

fn empty_arguments() -> Value {
    json!({})
}
