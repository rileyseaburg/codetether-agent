use super::*;

#[test]
fn test_topic_matches() {
    assert!(topic_matches("agent.123.events", "*"));
    assert!(topic_matches("agent.123.events", "agent.*"));
    assert!(topic_matches("ralph.prd1", "ralph.*"));
    assert!(!topic_matches("task.42", "agent.*"));
    assert!(topic_matches("agent.123.events", "agent.123.events"));
}
