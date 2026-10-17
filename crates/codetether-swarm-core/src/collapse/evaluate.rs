use super::*;
impl CollapseController {
    pub(super) fn evaluate(
        &self,
        observation: &BranchObservation,
        snapshot: &ObservationSnapshot,
    ) -> BranchEvaluation {
        let score = self.evaluate_score(observation, snapshot);

        BranchEvaluation {
            subtask_id: observation.subtask_id.clone(),
            branch: observation.branch.clone(),
            aggregate_score: score.collapse_score(),
            score,
        }
    }
}
