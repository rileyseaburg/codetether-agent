//! Preserve provider-step preparation order and full textual-tool definitions.
use crate::{Host, ProviderStepState};

/// Derive one provider request's registry, schemas, prompt and temperature.
/// # Arguments
/// `host` supplies application services; `selected_provider` and `model`
/// identify the resolved provider and model.
/// # Returns
/// All derived settings, preserving the full catalog for textual-tool prompts.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_request_preparation::Host>(host: &H) {
/// let state = codetether_request_preparation::build_provider_step_state(host, "openai", "model");
/// let _prompt = state.system_prompt;
/// # }
/// ```
pub fn build_provider_step_state<H: Host>(
    host: &H,
    selected_provider: &str,
    model: &str,
) -> ProviderStepState<H::Registry> {
    let (cwd, prior_context_allowed, autonomous) = host.context();
    let tool_registry = host.registry(&cwd, prior_context_allowed, autonomous);
    let tool_definitions = host.active(&tool_registry, selected_provider);
    let model_supports_tools = crate::model_supports_tools(selected_provider, model);
    let advertised_tool_definitions =
        host.advertised(model_supports_tools, &tool_definitions, selected_provider);
    let system_prompt = crate::system_prompt_for(
        selected_provider,
        model_supports_tools,
        // Textual-tool providers need the full definitions, not the
        // advertised list, which is empty when native tools are off.
        &tool_definitions,
        &cwd,
        prior_context_allowed,
    );
    let system_prompt = host.compose(&system_prompt);

    ProviderStepState {
        tool_registry,
        tool_definitions,
        advertised_tool_definitions,
        temperature: codetether_provider_policy::temperature_for(model),
        model_supports_tools,
        system_prompt,
    }
}
