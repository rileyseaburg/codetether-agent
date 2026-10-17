use super::*;
impl CollapseController {
    /// Score observations and update pruning and promotion state.
    ///
    /// Empty input returns an empty tick without clearing prior controller state.
    ///
    /// # Arguments
    /// * `observations` - Backend health and change measurements in evaluation order.
    ///
    /// # Returns
    /// Evaluations, kill decisions and a promoted task, retaining at least one candidate.
    ///
    /// # Examples
    /// ```
    /// let mut controller = codetether_swarm_core::collapse::CollapseController::new(Default::default());
    /// assert!(controller.sample_observations(&[]).kills.is_empty());
    /// ```
    pub fn sample_observations(&mut self, observations: &[BranchObservation]) -> CollapseTick {
        if observations.is_empty() {
            return CollapseTick::default();
        }

        let active: HashSet<&str> = observations.iter().map(|b| b.subtask_id.as_str()).collect();
        self.first_seen_at
            .retain(|id, _| active.contains(id.as_str()));
        self.consecutive_compile_failures
            .retain(|id, _| active.contains(id.as_str()));
        self.consecutive_resource_failures
            .retain(|id, _| active.contains(id.as_str()));
        self.invariant_breaks
            .retain(|id, _| active.contains(id.as_str()));

        let snapshot = self.observe(observations);
        let mut evaluations = Vec::with_capacity(observations.len());
        for observation in observations {
            evaluations.push(self.evaluate(observation, &snapshot));
        }
        self.derive_decisions(evaluations)
    }
}
