use super::{Message, StepFunProvider};
use serde_json::Value;

impl StepFunProvider {
    // Request-only bridge: response content remains the original Option<String>.
    /// Convert history with the existing image and packed-reply handling.
    ///
    /// # Arguments
    /// * messages - Stored conversation messages.
    ///
    /// # Returns
    /// Serialized messages with transport-only image companions.
    ///
    /// # Panics
    /// Panics if serializing an internal ChatMessage fails.
    ///
    /// # Examples
    ///
    ///     let provider = codetether_provider_stepfun::StepFunProvider::new(String::new()).unwrap();
    ///     assert!(provider.convert_messages(&[]).is_empty());
    pub fn convert_messages(&self, messages: &[Message]) -> Vec<Value> {
        codetether_provider_core::chat_images::convert_many(messages, |message| {
            self.convert_text_messages(std::slice::from_ref(message))
                .into_iter()
                .map(|value| {
                    serde_json::to_value(value).expect("ChatMessage contains JSON-safe fields")
                })
                .collect()
        })
    }
}
