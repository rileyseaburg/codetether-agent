//! Native immutable session view for proactive orchestration.
use crate::{
    provider::{Message, Provider},
    rlm::RlmConfig,
    session::Session,
};
use std::sync::Arc;
impl codetether_proactive_context::Source for Session {
    fn id(&self) -> &str {
        &self.id
    }
    fn messages(&self) -> &[Message] {
        &self.messages
    }
    fn message_offset(&self) -> usize {
        Session::message_offset(self)
    }
    fn rlm_config(&self) -> &RlmConfig {
        &self.metadata.rlm
    }
    fn subcall_provider(&self) -> Option<&Arc<dyn Provider>> {
        self.metadata.subcall_provider.as_ref()
    }
    fn subcall_model_name(&self) -> Option<&str> {
        self.metadata.subcall_model_name.as_deref()
    }
}
