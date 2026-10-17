use super::*;
impl CollapseController {
    pub(super) fn observe_health(&mut self, observation: &BranchObservation) {
        let fail_counter = self
            .consecutive_compile_failures
            .entry(observation.subtask_id.clone())
            .or_insert(0);
        if observation.compile_ok {
            *fail_counter = 0;
        } else {
            *fail_counter += 1;
            *self
                .invariant_breaks
                .entry(observation.subtask_id.clone())
                .or_insert(0) += 1;
        }

        let infra_fail_counter = self
            .consecutive_resource_failures
            .entry(observation.subtask_id.clone())
            .or_insert(0);
        if observation.resource_health_score < self.policy.min_resource_health_score {
            *infra_fail_counter += 1;
        } else {
            *infra_fail_counter = 0;
        }
        if observation.infra_unhealthy_signals > 0 {
            *self
                .invariant_breaks
                .entry(observation.subtask_id.clone())
                .or_insert(0) += observation.infra_unhealthy_signals;
        }
    }
}
