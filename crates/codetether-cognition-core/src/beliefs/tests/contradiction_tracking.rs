//! contradiction tracking.
use super::*;
#[test]
fn contradiction_tracking() {
    let mut b1 = Belief {
        id: "b1".to_string(),
        belief_key: "api-stable".to_string(),
        claim: "API is stable".to_string(),
        confidence: 0.8,
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

    // Contest reduces confidence
    b1.confidence = (b1.confidence - 0.1).max(0.05);
    b1.contested_by.push("b2".to_string());
    assert!((b1.confidence - 0.7).abs() < f32::EPSILON);
    assert_eq!(b1.contested_by, vec!["b2".to_string()]);
}
