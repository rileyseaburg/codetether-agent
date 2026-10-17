//! Worker interfaces and session classification snapshots.
#[test]
fn worker_wire_and_classification() {
    use crate::codetether_agent::subagent_orphans::classify_child_session_value;
    let mut cases = Vec::new();
    for messages in 0..=2 {
        for tools in 0..=1 {
            for tokens in [0, 1] {
                for heartbeat in [false, true] {
                    let mut value = serde_json::json!({"id":"child","metadata":{"provenance":{"identity":{"agent_name":"nested"}}},
            "messages":vec![serde_json::json!({});messages],"tool_uses":vec![serde_json::json!({});tools],"usage":{"total_tokens":tokens}});
                    if heartbeat {
                        value["last_heartbeat_at"] = "now".into();
                    }
                    let record = classify_child_session_value(&value).unwrap();
                    cases.push(serde_json::to_value(record).unwrap());
                }
            }
        }
    }
    assert!(classify_child_session_value(&serde_json::json!({"agent":"missing-id"})).is_none());
    println!("WORKER_STATES={}", serde_json::to_string(&cases).unwrap());
    for url in [None, Some("  "), Some(" https://worker.example/// ")] {
        println!("WORKER_INTERFACE={}", crate::advertised_interfaces(url));
    }
    assert_eq!(
        crate::advertised_interfaces(Some("https://worker.example/"))["http"]["base_url"],
        "https://worker.example"
    );
}
mod cognition;
