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
    let live_claim = super::claim::assess_claim("Argo validation");
    let recovery = super::recovery::recovery_for(&live_claim, true);
    let workflow = super::workflow::infer("Argo platform upload");
    let guardrails = super::guardrails::render();
    let capabilities = super::capabilities::render();
    let trapdoor = super::trapdoor::render();
    let memory_protocol = super::memory_protocol::render();
    let core = cwd
        .map(super::core_memory::render)
        .unwrap_or_else(|| super::core_memory::storage_hint().to_string());
    let ids = if live_claim.needs_concrete_id {
        format!(
            "\nRuntime classifier: {} claims need concrete IDs; recovery action: {recovery:?}; gate: {}.",
            live_claim.level.label(),
            workflow.required_gate(),
        )
    } else {
        String::new()
    };
    super::assembly::render(super::assembly::PromptSections {
        system_prompt,
        guardrails,
        capabilities,
        trapdoor,
        memory_protocol,
        core,
        ids,
    })
}
