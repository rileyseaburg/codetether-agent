//! Goal-maintenance instructions for delegated and read-only agents.

use super::{SystemPromptInput, system_prompt};

#[test]
fn delegated_prompts_require_goal_progress_without_relaxing_tool_restrictions() {
    for (read_only, expects_changes) in [(false, true), (false, false), (true, false)] {
        let prompt = system_prompt(SystemPromptInput {
            specialty: "builder",
            subtask_id: "goal-maintenance",
            working_dir: ".",
            model: "provider/model",
            instruction: "Pursue the assigned outcome",
            context: "",
            line_limit: None,
            read_only,
            expects_changes,
        });
        assert!(prompt.contains("Before ending every turn"));
        assert!(prompt.contains("`session_task` action `reaffirm`"));
        assert!(prompt.contains("in your own session, not the parent's goal"));
        assert!(prompt.contains("do not bypass read-only or delegated tool restrictions"));
        assert!(
            prompt.contains("reserve `create_goal` and budget changes for explicit user requests")
        );
    }
}
