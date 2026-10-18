//! Remove approval transport metadata before comparing user-declined requests.
use serde_json::Value;

pub(super) fn normalized(input: &Value) -> Value {
    let mut value = input.clone();
    if let Some(values) = value.as_object_mut() {
        for field in ["approval_id", "_tool_call_id", "justification"] {
            values.remove(field);
        }
    }
    value
}
