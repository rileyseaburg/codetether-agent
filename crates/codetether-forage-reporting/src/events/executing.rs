//! Forage executing reporting.
use super::*;
impl RunEvents<'_> {
    /// Publish the working state and return the execution task ID.
    pub fn executing(&self, item: &ForageOpportunity) -> String {
        tracing::info!(target: "codetether_agent::forage",
            okr_id = %item.okr_id,
            key_result_id = %item.key_result_id,
            engine = %self.args.execution_engine,
            score = item.score,
            "Executing forage opportunity"
        );

        let exec_task_id = format!("forage-okr-{}-kr-{}", item.okr_id, item.key_result_id);
        let _ = self.bus_handle.send_task_update(
            &exec_task_id,
            TaskState::Working,
            Some(format!(
                "executing ({}) opportunity '{}' for KR '{}'",
                self.args.execution_engine, item.okr_title, item.key_result_title
            )),
        );

        exec_task_id
    }
}
