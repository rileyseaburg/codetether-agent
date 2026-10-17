//! confidence clamping.
use super::*;
#[test]
fn confidence_clamping() {
    let mut belief = Belief {
        id: "b1".to_string(),
        belief_key: "test-key".to_string(),
        claim: "test claim".to_string(),
        confidence: 1.5,
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
    belief.clamp_confidence();
    assert!((belief.confidence - 0.95).abs() < f32::EPSILON);

    belief.confidence = -0.5;
    belief.clamp_confidence();
    assert!((belief.confidence - 0.05).abs() < f32::EPSILON);
}
