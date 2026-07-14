use super::evaluate_tool_invocation_with_config;
use crate::config::Config;
use serde_json::json;

#[test]
fn session_ceiling_blocks_unenforced_agent_loops() {
    for (tool, action) in [
        ("go", "execute"),
        ("ralph", "run"),
        ("relay_autochat", "delegate"),
        ("relay_autochat", "handoff"),
    ] {
        let args = json!({
            "action": action,
            "__ct_prior_context_allowed": false,
        });
        let result =
            evaluate_tool_invocation_with_config(&Config::default(), tool, &args).expect("blocked");
        assert_eq!(
            result.metadata["error_code"],
            "PRIOR_CONTEXT_DISABLED_BY_USER"
        );
    }
}
