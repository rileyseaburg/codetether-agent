//! Focused tests for AGENTS.md prompt byte caps.

use super::super::{load_all_agents_md, load_all_agents_md_with_byte_cap};
use tempfile::tempdir;

#[test]
fn default_byte_cap_is_32_kib() {
    let tmp = tempdir().expect("tempdir");
    let repo = tmp.path().join("repo");
    std::fs::create_dir_all(repo.join(".git")).expect("create .git dir");
    let body = "a".repeat(32 * 1024 + 1);
    std::fs::write(repo.join("AGENTS.md"), body).expect("write AGENTS");

    let loaded = load_all_agents_md(&repo);

    assert_eq!(loaded[0].0.len(), 32 * 1024);
}

#[test]
fn custom_byte_cap_applies_across_root_to_leaf_merge() {
    let tmp = tempdir().expect("tempdir");
    let repo = tmp.path().join("repo");
    let child = repo.join("child");
    std::fs::create_dir_all(repo.join(".git")).expect("create .git dir");
    std::fs::create_dir_all(&child).expect("create child");
    std::fs::write(repo.join("AGENTS.md"), "abcdef").expect("write root");
    std::fs::write(child.join("AGENTS.md"), "ghij").expect("write child");

    let loaded = load_all_agents_md_with_byte_cap(&child, 8);

    assert_eq!(loaded[0].0, "abcdef");
    assert_eq!(loaded[1].0, "gh");
    assert_eq!(
        loaded
            .iter()
            .map(|(content, _)| content.len())
            .sum::<usize>(),
        8
    );
}
