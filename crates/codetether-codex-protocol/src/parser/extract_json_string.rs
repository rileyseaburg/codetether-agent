//! extract json string.
use crate::*;
impl CodexProtocol {
    /// Clone string values or serialize other JSON values into argument text.
    pub fn extract_json_string(value: Option<&Value>) -> Option<String> {
        match value {
            Some(Value::String(text)) => Some(text.clone()),
            Some(other) => serde_json::to_string(other).ok(),
            None => None,
        }
    }
}
