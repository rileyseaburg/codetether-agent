//! Authorization-relevant patch execution mode.

use serde_json::Value;

pub(super) fn for_args(resource: String, args: &Value) -> String {
    let preview = ["dry_run", "preview"]
        .iter()
        .any(|key| args.get(key).and_then(Value::as_bool) == Some(true));
    select(resource, preview)
}

pub(super) fn select(resource: String, preview: bool) -> String {
    format!(
        "{resource}::mode={}",
        if preview { "preview" } else { "apply" }
    )
}

pub(super) fn apply(resource: String) -> String {
    select(resource, false)
}
