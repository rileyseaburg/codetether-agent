use super::{Message, StepFunProvider};
use serde_json::Value;

impl StepFunProvider {
    // Request-only bridge: response content remains the original Option<String>.
    pub(in crate::provider) fn convert_messages(&self, messages: &[Message]) -> Vec<Value> {
        crate::provider::chat_images::convert_many(messages, |message| {
            self.convert_text_messages(std::slice::from_ref(message))
                .into_iter()
                .map(|value| {
                    serde_json::to_value(value).expect("ChatMessage contains JSON-safe fields")
                })
                .collect()
        })
    }
}
