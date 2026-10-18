use super::evaluate_tool_invocation_with_config;
use crate::config::Config;
use serde_json::json;

#[test]
fn scoped_git_inspection_is_read_only_but_mutation_is_not() {
    let config = Config::default();
    for command in [
        "git -C /workspace status --short",
        "git -C ./repo diff --stat",
        "git -C repo rev-parse --show-toplevel",
    ] {
        let args = json!({"command": command});
        assert!(
            evaluate_tool_invocation_with_config(&config, "bash", &args).is_none(),
            "{command} should be read-only"
        );
    }
    let args = json!({"command": "git -C /workspace commit -m change"});
    assert!(evaluate_tool_invocation_with_config(&config, "bash", &args).is_some());
}
