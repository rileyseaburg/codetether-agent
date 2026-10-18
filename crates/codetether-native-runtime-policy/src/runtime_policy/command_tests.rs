use super::evaluate_tool_invocation_with_config;
use crate::config::{Config, PermissionAction};
use serde_json::json;

#[test]
fn read_only_bash_command_is_allowed_without_approval() {
    let args = json!({"command": "date"});
    assert!(evaluate_tool_invocation_with_config(&Config::default(), "bash", &args).is_none());
}

#[test]
fn command_prefix_allow_rule_bypasses_default_bash_approval() {
    let mut config = Config::default();
    config
        .permissions
        .rules
        .insert("cargo test".into(), PermissionAction::Allow);
    let args = json!({"command": "cargo test --lib command_tests"});
    assert!(evaluate_tool_invocation_with_config(&config, "bash", &args).is_none());
}

#[test]
fn command_prefix_deny_rule_blocks_even_safe_command() {
    let mut config = Config::default();
    config
        .permissions
        .rules
        .insert("git status".into(), PermissionAction::Deny);
    let args = json!({"command": "git status --short"});
    let result = evaluate_tool_invocation_with_config(&config, "bash", &args).expect("blocked");
    assert!(!result.success);
}

#[test]
fn read_only_bash_rejects_compound_shell_syntax() {
    assert_requires_approval("ls && rm -rf /");
    assert_requires_approval("git status && rm -rf /");
}

#[test]
fn read_only_bash_rejects_mutating_find_and_git_flags() {
    assert_requires_approval("find . -name file.txt -delete");
    assert_requires_approval("find . -exec rm {} ;");
    assert_requires_approval("git diff --output=/tmp/diff.txt");
}

fn assert_requires_approval(command: &str) {
    let args = json!({"command": command});
    let result = evaluate_tool_invocation_with_config(&Config::default(), "bash", &args);
    assert!(result.is_some(), "{command} should not be read-only");
}
