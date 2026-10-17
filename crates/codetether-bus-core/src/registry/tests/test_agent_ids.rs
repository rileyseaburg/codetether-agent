use super::*;

#[test]
fn test_agent_ids() {
    let reg = AgentRegistry::new();
    reg.create_ephemeral("alpha", "a", vec![]);
    reg.create_ephemeral("beta", "b", vec![]);
    let mut ids = reg.agent_ids();
    ids.sort();
    assert_eq!(ids, vec!["alpha", "beta"]);
}
