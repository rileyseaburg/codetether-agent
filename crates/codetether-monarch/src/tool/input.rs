//! Typed inputs for the read-only Monarch facade.
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum Input {
    Status {},
    Refresh {},
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

/// Separate runtime envelope fields from the strictly validated public input.
pub(super) fn parse(mut args: Value) -> serde_json::Result<Input> {
    if let Some(fields) = args.as_object_mut() {
        // Do not recurse: nested arguments belong to the remote tool schema.
        fields.retain(|key, _| !key.starts_with("__ct_") && key != "_tool_call_id");
    }
    serde_json::from_value(args)
}
