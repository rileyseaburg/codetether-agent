use std::sync::Arc;

use crate::provider::Provider;
use crate::session::Session;

use super::workspace_tools::registry_for_cwd;

#[path = "request_state/apply_to.rs"]
mod apply_to;
#[path = "request_state/session_context.rs"]
mod session_context;
#[path = "request_state/settings.rs"]
pub(crate) mod settings;
#[path = "request_state/state.rs"]
mod state;
#[path = "request_state/tools.rs"]
mod tools;

pub(crate) use state::ProviderStepState;

pub(crate) fn build_provider_step_state(
    provider: Arc<dyn Provider>,
    selected_provider: &str,
    model: &str,
    session: &Session,
) -> ProviderStepState {
    let (cwd, prior_context_allowed, autonomous) = session_context::resolve(session);
    let tool_registry = registry_for_cwd(provider, model, &cwd, prior_context_allowed, autonomous);
    let tool_definitions = tools::active_tool_definitions(&tool_registry, selected_provider);
    let model_supports_tools = settings::model_supports_tools(selected_provider);
    let advertised_tool_definitions =
        tools::advertised_tools(model_supports_tools, &tool_definitions);
    let system_prompt = settings::system_prompt_for(
        selected_provider,
        model_supports_tools,
        &advertised_tool_definitions,
        &cwd,
        prior_context_allowed,
    );

    ProviderStepState {
        tool_registry,
        tool_definitions,
        advertised_tool_definitions,
        temperature: settings::temperature_for(model),
        model_supports_tools,
        system_prompt,
    }
}
