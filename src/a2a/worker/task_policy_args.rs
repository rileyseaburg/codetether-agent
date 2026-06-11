//! Runtime policy invocation arguments for special worker tasks.

use serde_json::Value;

pub(super) fn from_metadata(metadata: &serde_json::Map<String, Value>) -> Value {
    let mut args = serde_json::Map::new();
    if let Some(value) = super::metadata_lookup(metadata, "approval_id") {
        args.insert("approval_id".to_string(), value.clone());
    }
    Value::Object(args)
}

#[cfg(test)]
#[path = "task_policy_args_tests.rs"]
mod tests;
