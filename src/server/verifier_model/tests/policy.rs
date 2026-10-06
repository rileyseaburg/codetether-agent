//! Mutations must match the write rule before the broad config-read rule.

#[test]
fn verifier_model_api_policy_requires_agent_write_for_mutations() {
    let path = "/api/config/verifier-model";
    assert_eq!(
        crate::server::match_policy_rule(path, "GET"),
        Some("agent:read")
    );
    for method in ["PUT", "DELETE"] {
        assert_eq!(
            crate::server::match_policy_rule(path, method),
            Some("agent:write")
        );
    }
    let policy: serde_json::Value =
        serde_json::from_str(include_str!("../../../../policies/data.json")).unwrap();
    let permissions = |role: &str| policy["roles"][role]["permissions"].as_array().unwrap();
    assert!(permissions("admin").iter().any(|p| p == "agent:write"));
    assert!(permissions("viewer").iter().any(|p| p == "agent:read"));
    assert!(!permissions("viewer").iter().any(|p| p == "agent:write"));
}
