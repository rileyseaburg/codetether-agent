//! Native read-only session binding for shared legacy derivation.
impl codetether_context_legacy::Source for crate::session::Session {
    fn messages(&self) -> &[crate::provider::Message] {
        &self.messages
    }
    fn session_id(&self) -> &str {
        &self.id
    }
    fn rlm_config(&self) -> &crate::rlm::RlmConfig {
        &self.metadata.rlm
    }
}
