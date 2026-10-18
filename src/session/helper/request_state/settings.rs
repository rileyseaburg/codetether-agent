use super::super::bootstrap::inject_tool_prompt;
use super::super::evidence::append_guardrails_for_cwd;
use super::super::runtime::{local_cuda_light_system_prompt, prefers_light_system_prompt};
pub(crate) use codetether_provider_policy::temperature_for;

use crate::provider::ToolDefinition;
use std::path::Path;

pub(super) fn model_supports_tools(provider: &str, model: &str) -> bool {
    // local_cuda has no native tool-call channel, so the caller injects a
    // textual `<tool_call>` contract and parses it back via
    // `markup::normalize_textual_tool_calls`.
    if matches!(provider, "local-cuda" | "local_cuda" | "localcuda") {
        return false;
    }
    provider != "openrouter"
        || crate::provider::openrouter::model_capabilities::supports_tools(model)
}

pub(super) fn system_prompt_for(
    selected_provider: &str,
    model_supports_tools: bool,
    advertised_tools: &[ToolDefinition],
    cwd: &Path,
    prior_context_allowed: bool,
) -> String {
    let prompt = if prefers_light_system_prompt(selected_provider) {
        crate::agent::builtin::append_project_instructions(local_cuda_light_system_prompt(), cwd)
    } else {
        crate::agent::builtin::build_system_prompt(cwd)
    };
    let prompt = append_guardrails_for_cwd(prompt, cwd, prior_context_allowed);
    let prompt = if !model_supports_tools && !advertised_tools.is_empty() {
        inject_tool_prompt(&prompt, advertised_tools)
    } else {
        prompt
    };
    crate::a2a::peer_prompt::append(prompt)
}

#[cfg(test)]
#[path = "settings_tests.rs"]
mod tests;
