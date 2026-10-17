//! Relay regression coverage.
use super::*;

#[tokio::test]
async fn list_agents_is_scoped_to_single_relay() {
    let tool = RelayAutoChatTool::new();
    let relay_a = relay_id("relay-a");
    let relay_b = relay_id("relay-b");

    init_relays::init_relays(&tool, relay_a.clone(), relay_b.clone()).await;
    tool.delegate_task(
        Some(relay_a.clone()),
        Some("agent-alpha".to_string()),
        Some("do a".to_string()),
        None,
        None,
    )
    .await
    .expect("delegate relay a");
    tool.delegate_task(
        Some(relay_b.clone()),
        Some("agent-beta".to_string()),
        Some("do b".to_string()),
        None,
        None,
    )
    .await
    .expect("delegate relay b");

    let result = tool
        .list_agents(Some(relay_a))
        .await
        .expect("list agents for relay a");
    assert!(result.success);
    let payload = parse_output_json(&result.output);
    let names: Vec<&str> = payload["agents"]
        .as_array()
        .expect("agents array")
        .iter()
        .filter_map(|agent| agent["name"].as_str())
        .collect();

    assert_eq!(names, vec!["agent-alpha"]);
    assert_eq!(payload["count"], json!(1));
}
