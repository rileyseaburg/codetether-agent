use super::{SystemPromptInput, system_prompt};

#[test]
fn approximate_refactor_size_triggers_fresh_measurement() {
    let prompt = system_prompt(SystemPromptInput {
        specialty: "refactor",
        subtask_id: "metrics",
        working_dir: ".",
        model: "provider/model",
        instruction: "Refactor the roughly 442-line GraphCanvas.tsx",
        context: "",
        line_limit: None,
        read_only: false,
        expects_changes: true,
    });

    assert!(prompt.contains("SOURCE METRIC CONTRACT"));
    assert!(prompt.contains("fresh baseline"));
    assert!(prompt.contains("never as current evidence"));
}
