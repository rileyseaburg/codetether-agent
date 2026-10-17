//! record completed reasoning.
use crate::*;
impl CodexProtocol {
    /// Preserve reasoning checkpoints found in the completed response.
    pub fn record_completed_reasoning(response: Option<&Value>, chunks: &mut Vec<StreamChunk>) {
        let Some(output) = response
            .and_then(|value| value.get("output"))
            .and_then(Value::as_array)
        else {
            return;
        };
        for item in output {
            crate::reasoning::push_item_if_reasoning(item, chunks);
        }
    }
}
