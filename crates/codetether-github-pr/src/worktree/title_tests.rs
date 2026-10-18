//! Existing title regression tests.

use super::*;

#[test]
fn title_prefers_commit_subject() {
    let commits = vec!["abc123 fix: clean PR title generation".into()];
    assert_eq!(
        build_title("tui_abc", &commits),
        "fix: clean PR title generation"
    );
}

#[test]
fn truncate_title_handles_unicode() {
    let result = truncate_title("fix 🦀 unicode title boundary", 8);
    assert_eq!(result, "fix");
}
