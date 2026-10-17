use super::*;

#[test]
fn derive_decisions_kills_repeated_failures_and_promotes_best() {
    let mut controller = CollapseController::new(CollapsePolicy::default());
    controller
        .consecutive_compile_failures
        .insert("bad".to_string(), 3);
    controller.invariant_breaks.insert("bad".to_string(), 2);

    let evals = vec![
        BranchEvaluation {
            subtask_id: "bad".to_string(),
            branch: "codetether/subagent-bad".to_string(),
            score: CoherenceScore {
                compile_health: 0.0,
                test_health: 0.0,
                contract_alignment: 0.0,
                diff_conflict_risk: 0.9,
                velocity: 0.1,
                resource_health_score: 0.2,
                invariant_breaks: 2,
            },
            aggregate_score: 0.0,
        },
        BranchEvaluation {
            subtask_id: "good".to_string(),
            branch: "codetether/subagent-good".to_string(),
            score: CoherenceScore {
                compile_health: 1.0,
                test_health: 0.6,
                contract_alignment: 0.9,
                diff_conflict_risk: 0.1,
                velocity: 0.4,
                resource_health_score: 1.0,
                invariant_breaks: 0,
            },
            aggregate_score: 0.82,
        },
    ];

    let tick = controller.derive_decisions(evals);
    assert_eq!(tick.kills.len(), 1);
    assert_eq!(tick.kills[0].subtask_id, "bad");
    assert_eq!(tick.promoted_subtask_id.as_deref(), Some("good"));
}
