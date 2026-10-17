//! Existing bus runtime regression coverage.
use super::*;

#[tokio::test]
async fn test_ready_shutdown_syncs_registry() {
    let bus = AgentBus::new().into_arc();
    let handle = bus.handle("planner-1");

    handle.announce_ready(vec!["plan".to_string(), "review".to_string()]);
    assert!(bus.registry.get("planner-1").is_some());

    handle.announce_shutdown();
    assert!(bus.registry.get("planner-1").is_none());
}
