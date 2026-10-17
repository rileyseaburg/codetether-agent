//! Await stage handles in their existing order and log task-join errors.
use super::parallel_result_types::ParallelStoryResult;
use super::*;
impl RalphLoop {
    pub(super) async fn collect_parallel_results(
        &mut self,
        handles: Vec<tokio::task::JoinHandle<ParallelStoryResult>>,
        provider: &Arc<dyn Provider>,
        model: &str,
        working_dir: &Path,
    ) {
        // Wait for all stories in this stage
        let _stage_passed = 0usize;
        for handle in handles {
            match handle.await {
                Ok(result) => {
                    self.apply_parallel_result(result, provider, model, working_dir)
                        .await
                }
                Err(e) => {
                    warn!("Story execution task failed: {}", e);
                }
            }
        }
    }
}
