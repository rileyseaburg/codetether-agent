//! Network classification for first-class collaboration mutations.

use serde_json::json;

#[test]
fn collaboration_mutations_are_network_governed() {
    for tool in [
        "spawn_agent",
        "followup_task",
        "resume_agent",
        "send_input",
        "send_message",
    ] {
        let args = json!({"__ct_effective_network_allowed":false});
        assert!(
            super::super::governed(tool, &args),
            "missing network gate: {tool}"
        );
        let decision = super::super::decision(tool, &args).expect("network denial");
        assert_eq!(
            decision.reason,
            super::super::DecisionReason::NetworkDisabled
        );
    }
    for tool in ["close_agent", "interrupt_agent"] {
        assert!(
            !super::super::governed(tool, &json!({})),
            "overclassified: {tool}"
        );
    }
}
