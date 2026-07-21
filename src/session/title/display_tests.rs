use super::display::{label, normalize_title};

#[test]
fn normalizes_persisted_titles() {
    assert_eq!(
        normalize_title("  Review\n session\x1b   storage ").as_deref(),
        Some("Review session storage")
    );
}

#[test]
fn labels_reject_bootstrap_context() {
    let title = "# AGENTS.md instructions for /tmp/project";
    assert_eq!(label(Some(title), "abcdef123456"), "abcdef12");
}

#[test]
fn labels_empty_sessions_as_new() {
    assert_eq!(label(Some("   "), ""), "new");
}
