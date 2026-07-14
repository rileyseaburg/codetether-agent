use std::path::Path;

pub(crate) fn append_guardrails_for_cwd(
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
