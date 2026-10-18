//! Existing TUI regression cases moved beside their implementation.
use crate::{command_with_optional_args, match_slash_command_hint, normalize_easy_command};

#[test]
fn command_with_optional_args_handles_bare_command() {
    assert_eq!(command_with_optional_args("/spawn", "/spawn"), Some(""));
}

#[test]
fn command_with_optional_args_handles_arguments() {
    assert_eq!(
        command_with_optional_args("/spawn planner you plan", "/spawn"),
        Some("planner you plan")
    );
}

#[test]
fn command_with_optional_args_ignores_prefix_collisions() {
    assert_eq!(command_with_optional_args("/spawned", "/spawn"), None);
}

#[test]
fn command_with_optional_args_ignores_autochat_prefix_collisions() {
    assert_eq!(command_with_optional_args("/autochatty", "/autochat"), None);
}

#[test]
fn command_with_optional_args_trims_leading_whitespace_in_args() {
    assert_eq!(
        command_with_optional_args("/kill    local-agent-1", "/kill"),
        Some("local-agent-1")
    );
}

#[test]
fn slash_hint_includes_protocol_command() {
    let hint = match_slash_command_hint("/protocol");
    assert!(hint.contains("/protocol"));
}

#[test]
fn slash_hint_includes_autochat_command() {
    let hint = match_slash_command_hint("/autochat");
    assert!(hint.contains("/autochat"));
}

#[test]
fn slash_hint_includes_autochat_local_command() {
    let hint = match_slash_command_hint("/autochat-local");
    assert!(hint.contains("/autochat-local"));
}

#[test]
fn slash_hint_includes_file_command() {
    let hint = match_slash_command_hint("/file");
    assert!(hint.contains("/file"));
}

#[test]
fn normalize_easy_command_preserves_okr_gated_go() {
    assert_eq!(
        normalize_easy_command("/go build a calculator"),
        "/go build a calculator"
    );
}

#[test]
fn normalize_easy_command_does_not_reinterpret_go_count() {
    assert_eq!(
        normalize_easy_command("/go 4 build a calculator"),
        "/go 4 build a calculator"
    );
}

#[test]
fn normalize_easy_command_keeps_count_only_as_an_okr_task() {
    assert_eq!(normalize_easy_command("/go 4"), "/go 4");
}

#[test]
fn slash_hint_handles_command_with_args() {
    let hint = match_slash_command_hint("/go 4");
    assert!(hint.contains("/go"));
}
