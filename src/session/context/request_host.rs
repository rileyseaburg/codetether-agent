//! Native preparation and policy binding for shared request orchestration.
use crate::{provider::Provider, session::Session};
use std::sync::Arc;
impl codetether_context_request::Source for Session {
    fn session_id(&self) -> &str {
        &self.id
    }
    fn effective_policy(&self, model: &str) -> codetether_context_policy::derivation::DerivePolicy {
        super::super::policy::effective_policy(self, model)
    }
    fn prepare(&self, provider: Arc<dyn Provider>, model: &str) {
        crate::session::index_produce::proactive::prepare(self, provider, model);
    }
}
