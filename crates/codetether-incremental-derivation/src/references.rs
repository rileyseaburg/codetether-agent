//! Reserve and attach source references independently of generated summaries.
use codetether_context_policy::{pages::ResidencyLevel, projection::helpers::DerivedContext};
use codetether_provider_core::Message;
use codetether_request_tokens::estimate_tokens_for_messages;

pub(super) struct References(Vec<(&'static str, Message)>);
impl References {
    pub(super) fn new(messages: &[Message], budget: usize, offset: usize) -> Self {
        Self(
            [
                (
                    "user_request_reference",
                    super::request_reference::build(messages, budget, offset),
                ),
                (
                    "approval_feedback_reference",
                    super::feedback_reference::build(messages, budget, offset),
                ),
            ]
            .into_iter()
            .filter_map(|(tag, message)| message.map(|message| (tag, message)))
            .collect(),
        )
    }
    pub(super) fn cost(&self) -> usize {
        self.0
            .iter()
            .map(|(_, message)| estimate_tokens_for_messages(std::slice::from_ref(message)))
            .sum()
    }
    pub(super) fn prepend(self, derived: &mut DerivedContext) {
        for (tag, message) in self.0.into_iter().rev() {
            derived.messages.insert(0, message);
            derived.resolutions.insert(0, ResidencyLevel::Compressed);
            derived.provenance.push(tag.into());
        }
    }
}
