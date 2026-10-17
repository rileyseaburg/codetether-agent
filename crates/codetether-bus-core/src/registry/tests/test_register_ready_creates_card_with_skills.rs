use super::*;

#[test]
fn test_register_ready_creates_card_with_skills() {
    let reg = AgentRegistry::new();
    let caps = vec!["plan.tasks".to_string(), "tool.call".to_string()];
    let card = reg.register_ready("agent-ready-1", &caps);

    assert_eq!(card.name, "agent-ready-1");
    assert_eq!(card.url, "bus://local/agent-ready-1");
    assert_eq!(card.skills.len(), 2);
    assert!(reg.get("agent-ready-1").is_some());
}
