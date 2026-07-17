use super::super::build_system_prompt;

#[test]
fn build_prompt_requires_managed_worktree_location() {
    let workspace = tempfile::tempdir().expect("workspace");
    let prompt = build_system_prompt(workspace.path());
    assert!(prompt.contains("<workspace-root>/.codetether-worktrees/"));
    assert!(prompt.contains("Never create a worktree in `/tmp`"));
    assert!(prompt.contains("Do not run `git worktree add` directly"));
}
