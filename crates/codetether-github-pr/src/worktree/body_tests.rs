//! Existing body regression tests.

use super::*;

#[test]
fn body_wraps_prompt_in_fence() {
    let body = build_body(Some("fix login"), "- abc fix");
    assert!(body.contains("## Original request\n\n```text\nfix login\n```"));
}

#[test]
fn body_uses_longer_fence_for_backticks() {
    let body = build_body(Some("contains ``` fence"), "");
    assert!(body.contains("````text\ncontains ``` fence\n````"));
}

#[test]
fn body_with_nothing_has_summary() {
    let body = build_body(None, "");
    assert!(body.contains("Automated CodeTether TUI agent changes"));
}
