//! Relay regression coverage.
use super::*;

#[tokio::test]
async fn complete_relay_reports_unique_participant_count() {
    let tool = RelayAutoChatTool::new();
    let relay = relay_id("relay-complete");

    tool.init_relay(Some(relay.clone()), Some("task".to_string()), None, None)
        .await
        .expect("init relay");

    for target in ["agent-a", "agent-b", "agent-a"] {
        tool.delegate_task(
            Some(relay.clone()),
            Some(target.to_string()),
            Some("work".to_string()),
            None,
            None,
        )
        .await
        .expect("delegate to participant");
    }

    let result = tool
        .complete_relay(Some(relay))
        .await
        .expect("complete relay");
    assert!(result.success);
    let payload = parse_output_json(&result.output);
    assert_eq!(payload["aggregated_results"]["total_agents"], json!(2));
}
