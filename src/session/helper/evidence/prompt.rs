use std::path::Path;

pub(crate) fn append_guardrails(system_prompt: String) -> String {
    append_guardrails_with_memory(system_prompt, None)
}

pub(crate) fn append_guardrails_for_cwd(system_prompt: String, cwd: &Path) -> String {
    append_guardrails_with_memory(system_prompt, Some(cwd))
}

fn append_guardrails_with_memory(system_prompt: String, cwd: Option<&Path>) -> String {
    if system_prompt.contains("Validation evidence rules:") {
        return system_prompt;
    }
    super::assembly::render(super::sections::build(system_prompt, cwd))
}
