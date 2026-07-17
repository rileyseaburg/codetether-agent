//! Tests for built-in agent prompt helpers.

mod agents_md_cap;
mod agents_md_discovery;
mod agents_md_global;
mod build_prompt_quality;
mod worktree_prompt;

use super::build_system_prompt;
use tempfile::tempdir;

#[test]
fn build_system_prompt_includes_non_interactive_build_guardrail() {
    let tmp = tempdir().expect("tempdir");
    std::fs::create_dir_all(tmp.path().join(".git")).expect("create .git dir");
    let prompt = build_system_prompt(tmp.path());
    assert!(prompt.contains("do not ask the user for permission to continue"));
    assert!(prompt.contains("repository documentation or files as the source of truth"));
    assert!(prompt.contains("restriction persists until explicitly revoked"));
    assert!(!prompt.contains("call the `session_recall` tool BEFORE"));
}
