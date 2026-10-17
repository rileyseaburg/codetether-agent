//! referenced events collected.
use super::*;
#[test]
fn referenced_events_collected() {
    use crate::beliefs::{Belief, BeliefStatus};
    use chrono::Duration;

    let mut beliefs = HashMap::new();
    beliefs.insert(
        "b1".to_string(),
        Belief {
            id: "b1".to_string(),
            belief_key: "test".to_string(),
            claim: "test".to_string(),
            confidence: 0.8,
            evidence_refs: vec!["evt-1".to_string(), "evt-2".to_string()],
            asserted_by: "p1".to_string(),
            confirmed_by: Vec::new(),
            contested_by: Vec::new(),
            contradicts: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            review_after: Utc::now() + Duration::hours(1),
            status: BeliefStatus::Active,
        },
    );

    let proposals = HashMap::new();
    let ids = referenced_event_ids(&beliefs, &proposals);
    assert!(ids.contains("evt-1"));
    assert!(ids.contains("evt-2"));
    assert_eq!(ids.len(), 2);
}
