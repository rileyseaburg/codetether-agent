use super::*;
impl CollapseController {
    pub(super) fn evaluate_score(
        &self,
        observation: &BranchObservation,
        snapshot: &ObservationSnapshot,
    ) -> CoherenceScore {
        let conflict_risk = conflict::conflict_risk(observation, snapshot);
        let compile_health = if *snapshot
            .compile_ok
            .get(&observation.subtask_id)
            .unwrap_or(&false)
        {
            1.0
        } else {
            0.0
        };

        // Phase 0 keeps test health lightweight to avoid expensive frequent test loops.
        let test_health = if compile_health > 0.0 { 0.6 } else { 0.0 };
        let contract_alignment = if compile_health > 0.0 {
            (1.0 - conflict_risk).clamp(0.0, 1.0)
        } else {
            0.0
        };

        let elapsed_secs = self
            .first_seen_at
            .get(&observation.subtask_id)
            .map(|t| t.elapsed().as_secs_f32().max(1.0))
            .unwrap_or(1.0);
        let lines = snapshot
            .changed_lines
            .get(&observation.subtask_id)
            .copied()
            .unwrap_or(0) as f32;
        let velocity = (lines / elapsed_secs / 20.0).clamp(0.0, 1.0);

        let invariant_breaks = *self
            .invariant_breaks
            .get(&observation.subtask_id)
            .unwrap_or(&0);
        CoherenceScore {
            compile_health,
            test_health,
            contract_alignment,
            diff_conflict_risk: conflict_risk,
            velocity,
            resource_health_score: observation.resource_health_score.clamp(0.0, 1.0),
            invariant_breaks,
        }
    }
}
