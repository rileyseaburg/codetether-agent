use super::*;
impl CollapseController {
    pub(super) fn derive_decisions(&mut self, evaluations: Vec<BranchEvaluation>) -> CollapseTick {
        if evaluations.is_empty() {
            self.promoted_subtask_id = None;
            return CollapseTick::default();
        }

        let (mut kills, mut killed_ids) = self.threshold_kills(&evaluations);
        // Never collapse every branch in a single sampling tick.
        if killed_ids.len() == evaluations.len()
            && let Some(best) = evaluations
                .iter()
                .max_by(|a, b| a.aggregate_score.total_cmp(&b.aggregate_score))
        {
            killed_ids.remove(&best.subtask_id);
            kills.retain(|k| k.subtask_id != best.subtask_id);
        }

        let alive: Vec<&BranchEvaluation> = evaluations
            .iter()
            .filter(|e| !killed_ids.contains(&e.subtask_id))
            .collect();

        self.prune(&alive, &mut killed_ids, &mut kills);
        let promoted_subtask_id = evaluations
            .iter()
            .filter(|e| !killed_ids.contains(&e.subtask_id))
            .max_by(|a, b| a.aggregate_score.total_cmp(&b.aggregate_score))
            .map(|e| e.subtask_id.clone());

        self.promoted_subtask_id = promoted_subtask_id.clone();

        CollapseTick {
            evaluations,
            kills,
            promoted_subtask_id,
        }
    }
}
