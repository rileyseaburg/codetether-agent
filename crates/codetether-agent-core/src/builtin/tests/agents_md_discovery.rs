//! Focused tests for AGENTS.md discovery ordering and precedence.

use super::super::{load_agents_md, load_all_agents_md};
use tempfile::tempdir;

#[test]
fn override_takes_precedence_per_directory() {
    let tmp = tempdir().expect("tempdir");
    let repo = tmp.path().join("repo");
    std::fs::create_dir_all(repo.join(".git")).expect("create .git dir");
    std::fs::write(repo.join("AGENTS.md"), "base").expect("write AGENTS");
    let override_path = repo.join("AGENTS.override.md");
    std::fs::write(&override_path, "override").expect("write override");

    let loaded = load_all_agents_md(&repo);

    assert_eq!(loaded, vec![("override".to_string(), override_path)]);
}

#[test]
fn load_all_agents_md_is_root_to_leaf_inside_repo() {
    let tmp = tempdir().expect("tempdir");
    let parent_agents = tmp.path().join("AGENTS.md");
    std::fs::write(&parent_agents, "parent").expect("write parent");
    let repo = tmp.path().join("repo");
    let subdir = repo.join("service");
    let nested = subdir.join("src");
    std::fs::create_dir_all(repo.join(".git")).expect("create .git dir");
    std::fs::create_dir_all(&nested).expect("create nested dir");
    let repo_agents = repo.join("AGENTS.md");
    let sub_agents = subdir.join("AGENTS.md");
    std::fs::write(&repo_agents, "repo").expect("write repo");
    std::fs::write(&sub_agents, "sub").expect("write sub");

    let loaded = load_all_agents_md(&nested);

    assert_eq!(loaded[0].1, repo_agents);
    assert_eq!(loaded[1].1, sub_agents);
    assert_eq!(load_agents_md(&nested).expect("closest").1, sub_agents);
    assert!(!loaded.iter().any(|(_, path)| path == &parent_agents));
}

#[test]
fn empty_instruction_files_are_skipped() {
    let tmp = tempdir().expect("tempdir");
    let repo = tmp.path().join("repo");
    std::fs::create_dir_all(repo.join(".git")).expect("create .git dir");
    std::fs::write(repo.join("AGENTS.override.md"), " \n").expect("write override");
    let agents_path = repo.join("AGENTS.md");
    std::fs::write(&agents_path, "base").expect("write AGENTS");

    let loaded = load_all_agents_md(&repo);

    assert_eq!(loaded, vec![("base".to_string(), agents_path)]);
}
