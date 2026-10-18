//! Compose project instructions, evidence, tool markup and live peer context.
use codetether_evidence_runtime::append_guardrails_for_cwd;
use codetether_prompt_policy::{local_cuda_light_system_prompt, prefers_light_system_prompt};
use codetether_provider_core::ToolDefinition;
use codetether_tool_catalog::inject_tool_prompt;
use std::path::Path;

/// Compose project, evidence, textual-tool and peer context for a request.
/// # Arguments
/// `selected_provider` chooses prompt size; `model_supports_tools` controls
/// markup instructions; `advertised_tools` supplies tool descriptions; `cwd`
/// scopes project context; `prior_context_allowed` gates memory access.
/// # Returns
/// The fully assembled system prompt in the original section order.
/// # Examples
/// ```no_run
/// let prompt = codetether_request_preparation::system_prompt_for(
///     "local_cuda", false, &[], std::path::Path::new("."), false
/// );
/// assert!(!prompt.is_empty());
/// ```
pub fn system_prompt_for(
    selected_provider: &str,
    model_supports_tools: bool,
    advertised_tools: &[ToolDefinition],
    cwd: &Path,
    prior_context_allowed: bool,
) -> String {
    let prompt = if prefers_light_system_prompt(selected_provider) {
        codetether_agent_core::builtin::append_project_instructions(
            local_cuda_light_system_prompt(),
            cwd,
        )
    } else {
        codetether_agent_core::builtin::build_system_prompt(cwd)
    };
    let prompt = append_guardrails_for_cwd(prompt, cwd, prior_context_allowed);
    let prompt = if !model_supports_tools && !advertised_tools.is_empty() {
        inject_tool_prompt(&prompt, advertised_tools)
    } else {
        prompt
    };
    codetether_a2a_discovery::peer_prompt::append(prompt)
}
