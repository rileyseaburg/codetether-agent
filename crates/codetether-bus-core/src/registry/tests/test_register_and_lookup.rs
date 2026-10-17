use super::*;

#[test]
fn test_register_and_lookup() {
    let reg = AgentRegistry::new();
    let card = reg.create_ephemeral("agent-1", "Test agent", vec![]);
    assert_eq!(reg.len(), 1);

    let found = reg.get("agent-1").unwrap();
    assert_eq!(found.url, card.url);
}
