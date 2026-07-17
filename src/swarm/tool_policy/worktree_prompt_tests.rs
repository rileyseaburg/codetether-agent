use super::{SystemPromptInput, system_prompt};

#[test]
fn subagent_prompt_requires_managed_worktree_location() {
    let prompt = system_prompt(SystemPromptInput {
        specialty: "builder",
        subtask_id: "one",
        working_dir: "/workspace/project",
        model: "provider/model",
        instruction: "Implement the change",
        context: "",
        line_limit: None,
        read_only: false,
        expects_changes: true,
    });
    assert!(prompt.contains("<workspace-root>/.codetether-worktrees/"));
    assert!(prompt.contains("Do not run `git worktree add` directly"));
}
