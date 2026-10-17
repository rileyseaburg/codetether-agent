//! duplicate detection by belief key.
use super::*;
#[test]
fn duplicate_detection_by_belief_key() {
    use std::collections::HashMap;

    let mut store: HashMap<String, Belief> = HashMap::new();
    let belief = Belief {
        id: "b1".to_string(),
        belief_key: "api-latency-high".to_string(),
        claim: "API latency is above SLA".to_string(),
        confidence: 0.7,
        evidence_refs: Vec::new(),
        asserted_by: "p1".to_string(),
        confirmed_by: Vec::new(),
        contested_by: Vec::new(),
        contradicts: Vec::new(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        review_after: Utc::now() + Duration::hours(1),
        status: BeliefStatus::Active,
    };
    store.insert(belief.id.clone(), belief);

    // Check duplicate detection
    let existing = store
        .values()
        .find(|b| b.belief_key == "api-latency-high" && b.status != BeliefStatus::Invalidated);
    assert!(existing.is_some());
}
