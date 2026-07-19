//! Pure mutation classification tests.

use serde_json::json;

#[test]
fn structured_tools_map_to_exact_paths() {
    let paths = super::paths::mutation_paths("write", &json!({ "path": "/work/a.rs" }));
    assert_eq!(paths.unwrap(), vec![std::path::PathBuf::from("/work/a.rs")]);

    let paths = super::paths::mutation_paths(
        "multiedit",
        &json!({ "edits": [{ "file": "a.rs" }, { "file": "b.rs" }] }),
    );
    assert_eq!(paths.unwrap().len(), 2);
}

#[test]
fn confirmed_and_preview_operations_only_lock_when_mutating() {
    assert!(super::paths::mutation_paths("confirm_edit", &json!({ "path": "a" })).is_none());
    let confirmed = json!({ "path": "a", "confirm": true });
    assert!(super::paths::mutation_paths("confirm_edit", &confirmed).is_some());
    let preview = json!({ "patch": "--- a/a\n+++ b/a", "preview": true });
    assert!(super::paths::mutation_paths("apply_patch", &preview).is_none());
}

#[test]
fn shell_classifier_only_allows_proven_read_commands() {
    assert!(super::shell::read_only("rg -n pattern src | head -20"));
    assert!(super::shell::read_only("git status --short"));
    assert!(!super::shell::read_only("cargo test"));
    assert!(!super::shell::read_only("rg pattern src > results.txt"));
    assert!(!super::shell::read_only("find . -delete"));
}

#[test]
fn unknown_shell_mutation_claims_the_workspace_root() {
    let paths = super::paths::mutation_paths("exec_command", &json!({ "cmd": "cargo fmt" }));
    assert_eq!(paths.unwrap(), vec![std::path::PathBuf::new()]);
}

#[test]
fn batch_aggregates_nested_mutations_atomically() {
    let input = json!({ "calls": [
        { "tool": "read", "args": { "path": "ignored" } },
        { "tool": "write", "args": { "path": "a.rs" } },
        { "name": "multiedit", "arguments": { "edits": [{ "file": "b.rs" }] } }
    ] });
    let paths = super::paths::mutation_paths("batch", &input).unwrap();
    assert_eq!(
        paths,
        vec![
            std::path::PathBuf::from("a.rs"),
            std::path::PathBuf::from("b.rs")
        ]
    );
}
