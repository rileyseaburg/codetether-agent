//! decay reduces confidence.
use super::*;
#[test]
fn decay_reduces_confidence() {
    let mut belief = Belief {
        id: "b1".to_string(),
        belief_key: "test-key".to_string(),
        claim: "test".to_string(),
        confidence: 0.5,
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
    belief.decay();
    assert!((belief.confidence - 0.49).abs() < f32::EPSILON);
}
