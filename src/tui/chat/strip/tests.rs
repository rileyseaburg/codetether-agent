use super::{has_tui_artifacts, strip_tui_artifacts};

#[test]
fn strips_full_box() {
    let raw = "┌──────────────┐\n│ hello world  │\n└──────────────┘";
    assert_eq!(strip_tui_artifacts(raw), "hello world");
}

#[test]
fn strips_leading_border_only() {
    assert_eq!(strip_tui_artifacts("│ hello world"), "hello world");
}

#[test]
fn strips_double_border_prefix() {
    assert_eq!(strip_tui_artifacts("││ tool output line"), "tool output line");
}

#[test]
fn preserves_clean_text() {
    let raw = "no borders here\njust plain text";
    assert_eq!(strip_tui_artifacts(raw), raw);
}

#[test]
fn collapses_multiple_blank_lines() {
    let raw = "│ line one │\n│          │\n│          │\n│ line two │";
    let result = strip_tui_artifacts(raw);
    assert!(result.contains("line one"));
    assert!(result.contains("line two"));
    assert!(!result.contains("\n\n\n"));
}

#[test]
fn has_artifacts_detects_box_chars() {
    assert!(has_tui_artifacts("│ hello │"));
    assert!(!has_tui_artifacts("plain text"));
}

#[test]
fn strips_real_tui_selection() {
    // Matches the exact format from the bug report.
    let raw = "│  The diagnostics are already resolved — the LSP reports no issues.  │\n└──────────────────────────────────────────────────────────────────────┘";
    let result = strip_tui_artifacts(raw);
    assert_eq!(result, "The diagnostics are already resolved — the LSP reports no issues.");
}
