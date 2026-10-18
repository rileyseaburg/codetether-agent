use crate::ProviderStepState;
use codetether_provider_core::Provider;
use std::sync::Arc;
/// Native session metadata, provider lookup and step preparation services.
/// # Implementors
/// The application binds its concrete session and tool-registry types.
pub trait Host<R, S> {
    /// Read the original provider/model selection stored in the session.
    fn original_model<'a>(&self, session: &'a S) -> Option<&'a str>;
    /// Resolve a configured provider by registry name.
    fn provider(&self, name: &str) -> Option<Arc<dyn Provider>>;
    /// Derive tool and prompt settings for the selected provider and model.
    fn build(
        &self,
        provider: Arc<dyn Provider>,
        name: &str,
        model: &str,
        session: &S,
    ) -> ProviderStepState<R>;
    /// Prepare proactive context after applying a failover.
    fn prepare(&self, session: &S, provider: Arc<dyn Provider>, model: &str);
}
