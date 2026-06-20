//! Regression tests for validation scope isolation (cross-thread safety).

use super::validation_scope;
use std::collections::HashSet;
use std::path::PathBuf;

fn set(paths: &[&str]) -> HashSet<PathBuf> {
    paths.iter().map(PathBuf::from).collect()
}

#[test]
fn scope_is_exactly_touched_files() {
    let touched = set(&["/ws/a.rs"]);
    let baseline = set(&[]);
    assert_eq!(validation_scope(&touched, &baseline), touched);
}

#[test]
fn foreign_dirty_file_is_never_in_scope() {
    // A file this agent never edited (dirtied by another agent / commit) must
    // not appear in the validation set, regardless of baseline contents.
    let touched = set(&["/ws/mine.rs"]);
    let baseline = set(&["/ws/foreign.tsx"]);
    let scope = validation_scope(&touched, &baseline);
    assert!(scope.contains(&PathBuf::from("/ws/mine.rs")));
    assert!(!scope.contains(&PathBuf::from("/ws/foreign.tsx")));
}

#[test]
fn empty_touched_yields_empty_scope() {
    assert!(validation_scope(&set(&[]), &set(&["/ws/other.rs"])).is_empty());
}