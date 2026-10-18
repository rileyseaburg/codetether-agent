//! Await stage handles in their existing order and log task-join errors.
use super::*;
pub(crate) async fn collect_parallel_results<H: Host>(
    host: &mut H,
    handles: Vec<tokio::task::JoinHandle<ParallelStoryResult>>,
    provider: &Arc<dyn Provider>,
    model: &str,
    working_dir: &Path,
) {
    // Wait for all stories in this stage
    let _stage_passed = 0usize;
    for handle in handles {
        match handle.await {
            Ok(result) => apply_parallel_result(host, result, provider, model, working_dir).await,
            Err(e) => {
                warn!(target: "codetether_agent::ralph::ralph_loop::parallel_collect", "Story execution task failed: {}", e);
            }
        }
    }
}
