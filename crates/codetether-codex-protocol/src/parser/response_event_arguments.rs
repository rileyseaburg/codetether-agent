//! response event arguments.
use crate::*;
impl CodexProtocol {
    /// Read final arguments when requested, otherwise read the streamed delta.
    pub fn response_event_arguments(event: &Value, final_arguments: bool) -> Option<String> {
        if final_arguments
            && let Some(arguments) = Self::extract_json_string(event.get("arguments"))
        {
            return Some(arguments);
        }
        event
            .get("delta")
            .and_then(Value::as_str)
            .map(str::to_string)
    }
}
