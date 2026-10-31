//! Shared worker prompts describe editing permission without forcing changes.
use crate::prompt_policy::{SystemPromptInput, system_prompt};
#[test]
fn shared_prompt_does_not_forbid_edits_or_require_unnecessary_changes() {
    let root = tempfile::tempdir().unwrap();
    let prompt = system_prompt(
        SystemPromptInput {
            specialty: "review",
            subtask_id: "one",
            working_dir: root.path().to_str().unwrap(),
            model: "mock/test",
            instruction: "Inspect code",
            context: "",
            line_limit: None,
            read_only: true,
            expects_changes: false,
            shared_workspace: true,
        },
        std::convert::identity,
    );
    assert!(prompt.contains("SHARED WRITABLE WORKSPACE"));
    assert!(prompt.contains("does not require unnecessary edits"));
    for forbidden in [
        "READ-ONLY TASK",
        "VERIFICATION TASK",
        "MUST use tools to make changes",
        "[runtime read-only mode]",
        "[runtime verification mode]",
    ] {
        assert!(!prompt.contains(forbidden), "{forbidden}");
    }
}
