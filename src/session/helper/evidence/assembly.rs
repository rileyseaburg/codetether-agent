pub(crate) struct PromptSections<'a> {
    pub system_prompt: String,
    pub guardrails: String,
    pub capabilities: String,
    pub trapdoor: &'a str,
    pub memory_protocol: &'a str,
    pub core: String,
    pub ids: String,
}

pub(crate) fn render(parts: PromptSections<'_>) -> String {
    format!(
        "{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}{}",
        parts.system_prompt,
        parts.guardrails,
        parts.capabilities,
        parts.trapdoor,
        parts.memory_protocol,
        parts.core,
        parts.ids
    )
}
