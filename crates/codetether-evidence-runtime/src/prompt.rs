//! Build evidence prompts with idempotent injection.
use std::path::Path;

/// Append evidence instructions and allowed workspace context exactly once.
/// # Arguments
/// `system_prompt` is the base prompt; `cwd` selects the workspace; and
/// `prior_context_allowed` gates project-memory and writeback reads.
/// # Returns
/// The original prompt if already decorated, otherwise the assembled prompt.
/// # Examples
/// ```no_run
/// let prompt = codetether_evidence_runtime::append_guardrails_for_cwd(
///     "Base".into(), std::path::Path::new("."), false
/// );
/// assert!(prompt.contains("Validation evidence rules:"));
/// ```
pub fn append_guardrails_for_cwd(
    system_prompt: String,
    cwd: &Path,
    prior_context_allowed: bool,
) -> String {
    append_guardrails_with_memory(system_prompt, Some(cwd), prior_context_allowed)
}

fn append_guardrails_with_memory(
    system_prompt: String,
    cwd: Option<&Path>,
    prior_context_allowed: bool,
) -> String {
    if system_prompt.contains("Validation evidence rules:") {
        return system_prompt;
    }
    super::assembly::render(super::sections::build(
        system_prompt,
        cwd,
        prior_context_allowed,
    ))
}
