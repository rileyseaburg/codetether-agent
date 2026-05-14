use std::path::Path;

pub(crate) fn build(
    system_prompt: String,
    cwd: Option<&Path>,
) -> super::assembly::PromptSections<'static> {
    super::assembly::PromptSections {
        system_prompt,
        guardrails: super::guardrails::render(),
        capabilities: super::capabilities::render(),
        trapdoor: super::trapdoor::render(),
        memory_protocol: super::memory_protocol::render(),
        workflow_templates: super::workflow_templates::render(),
        writeback: super::writeback::render(),
        core: core(cwd),
        prefetch: prefetch(cwd),
        ids: super::claim_note::render(),
    }
}

fn core(cwd: Option<&Path>) -> String {
    cwd.map(super::core_memory::render)
        .unwrap_or_else(|| super::core_memory::storage_hint().to_string())
}

fn prefetch(cwd: Option<&Path>) -> String {
    cwd.map(super::prefetch::render)
        .unwrap_or_else(|| "Runtime prefetch facts: cwd unavailable.".to_string())
}
