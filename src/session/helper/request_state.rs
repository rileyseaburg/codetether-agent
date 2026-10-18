//! Native services for shared provider-step preparation.
use crate::provider::Provider;
use crate::session::Session;
use std::sync::Arc;
#[path = "request_state/host.rs"]
mod host;
#[path = "request_state/session_context.rs"]
mod session_context;
#[path = "request_state/settings.rs"]
#[cfg(test)]
pub(crate) mod settings;
#[path = "request_state/tools.rs"]
mod tools;
pub(crate) type ProviderStepState =
    codetether_request_preparation::ProviderStepState<crate::tool::ToolRegistry>;
pub(crate) fn build_provider_step_state(
    provider: Arc<dyn Provider>,
    selected_provider: &str,
    model: &str,
    session: &Session,
) -> ProviderStepState {
    let host = host::NativeHost {
        provider,
        model,
        session,
    };
    codetether_request_preparation::build_provider_step_state(&host, selected_provider, model)
}
