//! Tests for built-in agent prompt helpers.

use super::{build_system_prompt, load_agents_md, load_all_agents_md};
use tempfile::tempdir;

#[test]
fn load_agents_md_stops_at_git_root() {
    let tmp = tempdir().expect("tempdir");
    let parent_agents = tmp.path().join("AGENTS.md");
    std::fs::write(&parent_agents, "parent instructions").expect("write parent AGENTS");

    let repo_root = tmp.path().join("repo");
    let nested = repo_root.join("src/nested");
    std::fs::create_dir_all(repo_root.join(".git")).expect("create .git dir");
    std::fs::create_dir_all(&nested).expect("create nested dir");
    let repo_agents = repo_root.join("AGENTS.md");
    std::fs::write(&repo_agents, "repo instructions").expect("write repo AGENTS");

    let loaded = load_agents_md(&nested).expect("expected AGENTS");
    assert_eq!(loaded.1, repo_agents);
    assert_eq!(loaded.0, "repo instructions");
}

#[test]
fn load_all_agents_md_collects_within_repo_only() {
    let tmp = tempdir().expect("tempdir");
    let parent_agents = tmp.path().join("AGENTS.md");
    std::fs::write(&parent_agents, "parent instructions").expect("write parent AGENTS");

    let repo_root = tmp.path().join("repo");
    let subdir = repo_root.join("service");
    let nested = subdir.join("src");
    std::fs::create_dir_all(repo_root.join(".git")).expect("create .git dir");
    std::fs::create_dir_all(&nested).expect("create nested dir");
    let repo_agents = repo_root.join("AGENTS.md");
    std::fs::write(&repo_agents, "repo instructions").expect("write repo AGENTS");
    let sub_agents = subdir.join("AGENTS.md");
    std::fs::write(&sub_agents, "sub instructions").expect("write sub AGENTS");

    let loaded = load_all_agents_md(&nested);
    assert_eq!(loaded.len(), 2);
    assert_eq!(loaded[0].1, sub_agents);
    assert_eq!(loaded[1].1, repo_agents);
    assert!(!loaded.iter().any(|(_, path)| path == &parent_agents));
}

#[test]
fn build_system_prompt_includes_non_interactive_build_guardrail() {
    let tmp = tempdir().expect("tempdir");
    std::fs::create_dir_all(tmp.path().join(".git")).expect("create .git dir");
    let prompt = build_system_prompt(tmp.path());
    assert!(prompt.contains("do not ask the user for permission to continue"));
}
