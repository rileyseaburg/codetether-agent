use serde_json::{Value, json};
use std::collections::HashMap;

pub(super) fn denied_metadata() -> HashMap<String, Value> {
    HashMap::from([("error_code".into(), json!("TOOL_APPROVAL_DENIED"))])
}
