use super::super::bootstrap::inject_tool_prompt;
use super::super::evidence::append_guardrails;
use super::super::provider::{prefers_temperature_one, temperature_is_deprecated};
use super::super::runtime::{is_local_cuda_provider, local_cuda_light_system_prompt};

use crate::provider::ToolDefinition;
use std::path::Path;

pub(super) fn temperature_for(model: &str) -> Option<f32> {
    if temperature_is_deprecated(model) {
        None
    } else if prefers_temperature_one(model) {
        Some(1.0)
    } else {
        Some(0.7)
    }
}

pub(super) fn model_supports_tools(provider: &str) -> bool {
    !matches!(
        provider,
        "gemini-web" | "local-cuda" | "local_cuda" | "localcuda"
    )
}

pub(super) fn system_prompt_for(
    selected_provider: &str,
    model_supports_tools: bool,
    advertised_tools: &[ToolDefinition],
    cwd: &Path,
) -> String {
    let prompt = if is_local_cuda_provider(selected_provider) {
        local_cuda_light_system_prompt()
    } else {
        crate::agent::builtin::build_system_prompt(cwd)
    };
    let prompt = append_guardrails(prompt);
    if !model_supports_tools && !advertised_tools.is_empty() {
        inject_tool_prompt(&prompt, advertised_tools)
    } else {
        prompt
    }
}
