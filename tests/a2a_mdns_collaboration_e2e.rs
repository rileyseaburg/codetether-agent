//! Proof that two zero-config runtimes become first-party callable peers.

use codetether_agent::a2a::spawn::start_a2a_in_background;
use codetether_agent::bus::AgentBus;
use codetether_agent::tool::Tool;
use codetether_agent::tool::agent::AgentTool;
use serde_json::json;

#[path = "a2a_mdns_collaboration/support.rs"]
mod support;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn zero_config_peers_appear_in_the_agent_tool() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("codetether_agent::a2a=info")
        .with_test_writer()
        .try_init();
    unsafe { std::env::set_var("CODETETHER_AUTH_TOKEN", "server-private") };
    let suffix = &uuid::Uuid::new_v4().simple().to_string()[..8];
    let alpha = format!("magic-alpha-{suffix}");
    let beta = format!("magic-beta-{suffix}");
    let first =
        start_a2a_in_background(support::options(alpha.clone()), AgentBus::new().into_arc())
            .await
            .unwrap();
    let second =
        start_a2a_in_background(support::options(beta.clone()), AgentBus::new().into_arc())
            .await
            .unwrap();
    assert!(first.bind_addr.starts_with("0.0.0.0:"));
    assert!(second.bind_addr.starts_with("0.0.0.0:"));
    assert!(!first.public_url.contains("127.0.0.1"));
    assert!(!second.public_url.contains("127.0.0.1"));
    let found = support::wait_for(&alpha, &beta).await;
    assert!(found.contains("a2a-mdns"), "LAN route missing: {found}");
    assert!(found.contains("CodeTether agent for workspace"));
    assert!(found.contains("\"skills\""));
    assert!(!found.contains("\"endpoint\""));
    unsafe { std::env::set_var("CODETETHER_AUTH_TOKEN", "client-wrong") };
    let reply = AgentTool::new().execute(json!({
        "action": "message", "name": beta,
        "message": "Hello from e2e (http://local). I am online and available for A2A collaboration."
    })).await.unwrap();
    assert!(
        reply.success,
        "discovered capability must authorize: {}",
        reply.output
    );
    assert!(reply.output.contains("A2A peer registered"));
    drop((first, second));
}
