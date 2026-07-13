use super::{SystemPromptInput, system_prompt};

#[test]
fn contradictory_task_gets_static_confidence_prompt() {
    let prompt = system_prompt(SystemPromptInput {
        specialty: "refactor",
        subtask_id: "conflict",
        working_dir: ".",
        model: "provider/model",
        instruction: "Preserve every behavior. Do not run tests, builds, compilers, or linters.",
        context: "",
        line_limit: None,
        read_only: false,
    });

    assert!(prompt.contains("VERIFICATION CONFLICT"));
    assert!(prompt.contains("label the result static/local"));
    assert!(prompt.contains("do not claim behavioral equivalence"));
    assert!(prompt.contains("[delegated task instruction]"));
}
