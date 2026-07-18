use codetether_agent::tool::{Tool, agent::AgentTool};
use serde_json::json;
use std::time::Duration;

pub async fn assert_remote_turn(name: &str) {
    let roster = tokio::time::timeout(Duration::from_secs(20), async {
        loop {
            let listed = AgentTool::new()
                .execute(json!({"action": "list"}))
                .await
                .unwrap();
            if listed.output.contains(name) {
                break listed.output;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .expect("independent peer never appeared in first-party agent list");
    assert!(roster.contains("CodeTether agent for workspace"));
    assert!(roster.contains("\"skills\""));
    assert!(!roster.contains("\"endpoint\""));
    unsafe { std::env::set_var("CODETETHER_AUTH_TOKEN", "observer-wrong") };
    let reply = AgentTool::new()
        .execute(json!({
            "action": "message", "name": name,
            "message": "Hello from process proof (http://local). I am online and available for A2A collaboration."
        }))
        .await
        .unwrap();
    assert!(
        reply.success,
        "first-party remote turn failed: {}",
        reply.output
    );
    assert!(reply.output.contains("A2A peer registered"));
    assert!(reply.output.contains("a2a-mdns"));
}
