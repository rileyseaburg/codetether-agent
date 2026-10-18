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
        expects_changes: true,
    });

    assert!(prompt.contains("VERIFICATION CONFLICT"));
    assert!(prompt.contains("label the result static/local"));
    assert!(prompt.contains("do not claim behavioral equivalence"));
    assert!(prompt.contains("[delegated task instruction]"));
}

#[test]
fn focused_checks_do_not_demand_source_edits() {
    let prompt = system_prompt(SystemPromptInput {
        specialty: "verifier",
        subtask_id: "tests",
        working_dir: ".",
        model: "provider/model",
        instruction: "Run focused tests",
        context: "",
        line_limit: None,
        read_only: false,
        expects_changes: false,
    });
    assert!(prompt.contains("VERIFICATION TASK"));
    assert!(!prompt.contains("MUST use tools to make changes"));
    assert!(!prompt.contains("COMPLEX TASKS"));
    assert!(
        prompt
            .trim_end()
            .ends_with("Never report a failed implementation method as task completion.")
    );
}
