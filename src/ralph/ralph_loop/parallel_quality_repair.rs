//! Execute one repair and append its output or stop on a repair failure.
use super::parallel_execution_context::ParallelExecutionContext;
use super::parallel_quality_state::QualityProgress;
use super::*;
impl ParallelExecutionContext<'_> {
    pub(super) async fn run_quality_repair(
        &self,
        state: &mut QualityProgress,
        error_output: &str,
        qg_attempt: usize,
        max_quality_retries: usize,
    ) -> bool {
        if !error_output.is_empty() {
            state.failure_reason = Some(error_output.to_string());
        }
        let repair_result = RalphLoop::repair_parallel_story(
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
                let output = &mut state.output;
                *output = format!(
                    "{output}\n---\n[repair attempt {}] {repair_output}",
                    qg_attempt + 1
                );
            }
            Err(e) => {
                warn!(
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
