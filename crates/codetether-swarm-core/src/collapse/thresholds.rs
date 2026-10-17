use super::*;
impl CollapseController {
    pub(super) fn threshold_kills(
        &self,
        evaluations: &[BranchEvaluation],
    ) -> (Vec<KillDecision>, HashSet<String>) {
        let mut kills = Vec::new();
        let mut killed_ids = HashSet::new();

        for eval in evaluations {
            let consecutive_failures = *self
                .consecutive_compile_failures
                .get(&eval.subtask_id)
                .unwrap_or(&0);
            let consecutive_resource_failures = *self
                .consecutive_resource_failures
                .get(&eval.subtask_id)
                .unwrap_or(&0);
            if eval.score.invariant_breaks >= self.policy.max_invariant_breaks
                || consecutive_failures >= self.policy.max_consecutive_compile_failures
                || consecutive_resource_failures >= self.policy.max_consecutive_resource_failures
            {
                killed_ids.insert(eval.subtask_id.clone());
                kills.push(KillDecision {
                    subtask_id: eval.subtask_id.clone(),
                    branch: eval.branch.clone(),
                    reason: format!(
                        "policy_threshold exceeded (invariant_breaks={}, consecutive_compile_failures={}, consecutive_resource_failures={}, resource_health_score={:.3})",
                        eval.score.invariant_breaks, consecutive_failures, consecutive_resource_failures, eval.score.resource_health_score
                    ),
                });
            }
        }

        (kills, killed_ids)
    }
}
