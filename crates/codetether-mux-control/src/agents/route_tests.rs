use serde_json::json;

#[test]
fn route_matching_ignores_the_human_display_name() {
    let item = json!({"agent_id": "spotless5", "name": "Morgan"});

    assert!(super::is_agent_route(&item, "spotless5"));
    assert!(!super::is_agent_route(&item, "Morgan"));
}
