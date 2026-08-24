use serde_json::json;
#[path = "network_collaboration_tests.rs"]
mod collaboration;
#[path = "network_receipt_tests.rs"]
mod receipt;
#[path = "network_trusted_state_tests.rs"]
mod trusted_state;

#[test]
fn disabled_web_tool_is_denied_before_approval() {
    let args = json!({
        "url": "https://example.com",
        "__ct_effective_network_allowed": false,
    });
    let decision = super::decision("webfetch", &args).expect("network decision");
    assert_eq!(decision.outcome, super::ToolPolicyOutcome::Deny);
    assert_eq!(decision.reason, super::DecisionReason::NetworkDisabled);
}

#[test]
fn provider_and_compatibility_tools_are_network_governed() {
    let args = json!({});
    for tool in [
        "imagegen",
        "kubernetes",
        "k8s_tool",
        "context_summarize",
        "rlm",
    ] {
        assert!(super::governed(tool, &args), "missing network gate: {tool}");
    }
}

#[test]
fn mixed_capability_tools_are_governed_only_for_network_actions() {
    assert!(super::governed("go", &json!({"action": "execute"})));
    assert!(!super::governed("go", &json!({"action": "status"})));
    assert!(super::governed(
        "relay_autochat",
        &json!({"action": "delegate"})
    ));
    assert!(!super::governed(
        "relay_autochat",
        &json!({"action": "status"})
    ));
}
