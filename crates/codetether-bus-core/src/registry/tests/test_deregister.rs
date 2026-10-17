use super::*;

#[test]
fn test_deregister() {
    let reg = AgentRegistry::new();
    reg.create_ephemeral("agent-2", "temp", vec![]);
    assert_eq!(reg.len(), 1);

    let removed = reg.deregister("agent-2");
    assert!(removed.is_some());
    assert_eq!(reg.len(), 0);
}
