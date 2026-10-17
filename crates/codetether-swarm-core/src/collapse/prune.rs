use super::*;
impl CollapseController {
    pub(super) fn prune(
        &self,
        alive: &[&BranchEvaluation],
        killed_ids: &mut HashSet<String>,
        kills: &mut Vec<KillDecision>,
    ) {
        // If enough candidates remain, prune one low-coherence branch.
        if alive.len() > 1 {
            let best = alive
                .iter()
                .max_by(|a, b| a.aggregate_score.total_cmp(&b.aggregate_score))
                .copied();
            let worst = alive
                .iter()
                .min_by(|a, b| a.aggregate_score.total_cmp(&b.aggregate_score))
                .copied();

            if let (Some(best), Some(worst)) = (best, worst) {
                let gap = best.aggregate_score - worst.aggregate_score;
                if worst.aggregate_score <= self.policy.min_collapse_score
                    && gap >= self.policy.min_score_gap_for_prune
                {
                    let newly_killed = killed_ids.insert(worst.subtask_id.clone());
                    if newly_killed {
                        kills.push(KillDecision {
                            subtask_id: worst.subtask_id.clone(),
                            branch: worst.branch.clone(),
                            reason: format!(
                                "low coherence pruned (score={:.3}, best={:.3}, gap={:.3})",
                                worst.aggregate_score, best.aggregate_score, gap
                            ),
                        });
                    }
                }
            }
        }
    }
}
