//! Execute one repair and append its output or stop on a repair failure.
use super::parallel_execution_context::ParallelExecutionContext;
use super::*;
impl<H: Host> ParallelExecutionContext<'_, H> {
    pub(crate) async fn run_quality_repair(
        &self,
        state: &mut QualityProgress,
        error_output: &str,
        qg_attempt: usize,
        max_quality_retries: usize,
    ) -> bool {
        state.remember_failure(error_output);
        let repair_result = crate::repair::repair_parallel_story::<H>(
            self.provider,
            self.model,
            self.story,
            self.working_dir,
            self.bridge_tx.clone(),
            self.bus.clone(),
            qg_attempt,
            max_quality_retries,
            error_output,
        )
        .await;
        match repair_result {
            Ok(repair_output) => {
                state.append_repair(repair_output, qg_attempt);
            }
            Err(e) => {
                warn!(target: "codetether_agent::ralph::ralph_loop::parallel_quality_repair",
                    story_id = %self.story.id,
                    error = %e,
                    "Repair agent failed"
                );
                return false;
            }
        }

        true
    }
}
