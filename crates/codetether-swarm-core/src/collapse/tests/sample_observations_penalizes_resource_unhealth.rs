use super::*;

#[test]
fn sample_observations_penalizes_resource_unhealth() {
    let mut controller = CollapseController::new(CollapsePolicy::default());
    let observations = vec![
        BranchObservation {
            subtask_id: "infra-bad".to_string(),
            branch: "codetether/subagent-infra-bad".to_string(),
            compile_ok: true,
            changed_files: HashSet::new(),
            changed_lines: 1,
            resource_health_score: 0.0,
            infra_unhealthy_signals: 2,
        },
        BranchObservation {
            subtask_id: "infra-good".to_string(),
            branch: "codetether/subagent-infra-good".to_string(),
            compile_ok: true,
            changed_files: HashSet::new(),
            changed_lines: 1,
            resource_health_score: 1.0,
            infra_unhealthy_signals: 0,
        },
    ];

    let tick = controller.sample_observations(&observations);
    let bad = tick
        .evaluations
        .iter()
        .find(|e| e.subtask_id == "infra-bad")
        .expect("infra-bad evaluation");
    let good = tick
        .evaluations
        .iter()
        .find(|e| e.subtask_id == "infra-good")
        .expect("infra-good evaluation");
    assert!(bad.aggregate_score < good.aggregate_score);
}
