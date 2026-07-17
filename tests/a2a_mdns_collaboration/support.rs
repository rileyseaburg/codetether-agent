use codetether_agent::a2a::spawn::SpawnOptions;
use codetether_agent::tool::{Tool, agent::AgentTool};
use serde_json::json;
use std::time::Duration;

pub fn options(name: String) -> SpawnOptions {
    SpawnOptions {
        name: Some(name),
        hostname: "0.0.0.0".to_string(),
        port: 0,
        public_url: None,
        description: None,
        peer: vec![],
        discovery_interval_secs: 5,
        auto_introduce: false,
        mdns: true,
    }
}

pub async fn wait_for(alpha: &str, beta: &str) -> String {
    let mut last = String::new();
    tokio::time::timeout(Duration::from_secs(15), async {
        loop {
            let listed = AgentTool::new()
                .execute(json!({ "action": "list" }))
                .await
                .unwrap();
            last = listed.output;
            if last.contains(alpha) && last.contains(beta) {
                break last.clone();
            }
            tokio::time::sleep(Duration::from_millis(250)).await;
        }
    })
    .await
    .unwrap_or_else(|_| panic!("peers missing; last agent list: {last}"))
}
