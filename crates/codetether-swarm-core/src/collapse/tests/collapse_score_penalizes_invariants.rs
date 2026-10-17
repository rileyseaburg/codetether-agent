use super::*;

#[test]
fn collapse_score_penalizes_invariants() {
    let healthy = CoherenceScore {
        compile_health: 1.0,
        test_health: 0.8,
        contract_alignment: 0.9,
        diff_conflict_risk: 0.1,
        velocity: 0.5,
        resource_health_score: 1.0,
        invariant_breaks: 0,
    };
    let broken = CoherenceScore {
        invariant_breaks: 3,
        ..healthy
    };

    assert!(healthy.collapse_score() > broken.collapse_score());
}
