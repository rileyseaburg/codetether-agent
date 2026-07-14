use std::path::Path;

pub(crate) fn build(
    system_prompt: String,
    cwd: Option<&Path>,
    prior_context_allowed: bool,
) -> super::assembly::PromptSections<'static> {
    super::assembly::PromptSections {
        system_prompt,
        guardrails: super::guardrails::render(),
        failure_attribution: super::failure_attribution::render(),
        capabilities: super::capabilities::render(),
        trapdoor: super::trapdoor::render(prior_context_allowed),
        memory_protocol: super::memory_protocol::render(prior_context_allowed),
        workflow_templates: super::workflow_templates::render(),
        writeback: super::writeback::render(prior_context_allowed),
        core: core(cwd, prior_context_allowed),
        prefetch: prefetch(cwd, prior_context_allowed),
        ids: super::claim_note::render(),
    }
}

fn core(cwd: Option<&Path>, allowed: bool) -> String {
    if !allowed {
        return "Core memory: not loaded at the user's request.".to_string();
    }
    cwd.map(super::core_memory::render)
        .unwrap_or_else(|| super::core_memory::storage_hint().to_string())
}

fn prefetch(cwd: Option<&Path>, allowed: bool) -> String {
    if !allowed {
        return "Runtime prefetch facts: prior-session data not loaded at the user's request."
            .to_string();
    }
    cwd.map(super::prefetch::render)
        .unwrap_or_else(|| "Runtime prefetch facts: cwd unavailable.".to_string())
}
