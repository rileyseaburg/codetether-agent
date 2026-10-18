//! Native model metadata, registry and preparation bindings.
use crate::{
    provider::{Provider, ProviderRegistry},
    session::Session,
    tool::ToolRegistry,
};
use std::sync::Arc;
pub(super) struct Native<'a>(pub &'a ProviderRegistry);
impl codetether_request_preparation::step_model::Host<ToolRegistry, Session> for Native<'_> {
    fn original_model<'a>(&self, session: &'a Session) -> Option<&'a str> {
        session.metadata.model.as_deref()
    }
    fn provider(&self, name: &str) -> Option<Arc<dyn Provider>> {
        self.0.get(name)
    }
    fn build(
        &self,
        provider: Arc<dyn Provider>,
        name: &str,
        model: &str,
        session: &Session,
    ) -> super::super::request_state::ProviderStepState {
        super::super::request_state::build_provider_step_state(provider, name, model, session)
    }
    fn prepare(&self, session: &Session, provider: Arc<dyn Provider>, model: &str) {
        super::step_prepare::run(session, provider, model);
    }
}
