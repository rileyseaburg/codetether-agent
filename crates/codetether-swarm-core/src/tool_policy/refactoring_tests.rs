//! Extraction tasks must receive both editing tools and matching worker instructions.
use crate::prompt_policy::{SystemPromptInput, system_prompt};
use crate::{SubTask, tool_policy};
use codetether_provider_core::ToolDefinition;

#[test]
fn extraction_receives_editing_tools_and_implementation_prompt() {
    let task = SubTask::new(
        "controller_zone",
        "Extract the controller into a crate. Replace raw SQL with a RustyRoad artifact. \
         Then inspect the result.",
    )
    .with_needs_worktree(false);
    let tools = [ToolDefinition {
        name: "apply_patch".into(),
        description: "Edit source files".into(),
        parameters: serde_json::json!({}),
    }];
    let available = tool_policy::definitions(&tools, task.is_read_only());
    assert_eq!(available.len(), 1);
    assert_eq!(available[0].name, "apply_patch");
    assert!(task.needs_worktree());

    let workspace = tempfile::tempdir().unwrap();
    let prompt = system_prompt(
        SystemPromptInput {
            specialty: "refactoring",
            subtask_id: &task.id,
            working_dir: workspace.path().to_str().unwrap(),
            model: "provider/model",
            instruction: &task.instruction,
            context: "",
            line_limit: None,
            read_only: task.is_read_only(),
            expects_changes: task.expects_file_changes(),
            shared_workspace: false,
        },
        std::convert::identity,
    );
    assert!(prompt.contains("You MUST use tools to make changes"));
    assert!(!prompt.contains("READ-ONLY TASK"));
    assert!(!prompt.contains("VERIFICATION TASK"));
}
