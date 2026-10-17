//! Existing relay regression coverage.
use super::*;

#[tokio::test]
async fn test_register_and_shutdown_agents() {
    let bus = AgentBus::new().into_arc();
    let relay = ProtocolRelayRuntime::with_relay_id(bus.clone(), "relay-test");

    let agents = vec![
        RelayAgentProfile {
            name: "auto-planner".to_string(),
            capabilities: vec!["planning".to_string()],
        },
        RelayAgentProfile {
            name: "auto-coder".to_string(),
            capabilities: vec!["coding".to_string()],
        },
    ];

    relay.register_agents(&agents);

    assert!(bus.registry.get("auto-planner").is_some());
    assert!(bus.registry.get("auto-coder").is_some());

    relay.shutdown_agents(&["auto-planner".to_string(), "auto-coder".to_string()]);

    assert!(bus.registry.get("auto-planner").is_none());
    assert!(bus.registry.get("auto-coder").is_none());
}
