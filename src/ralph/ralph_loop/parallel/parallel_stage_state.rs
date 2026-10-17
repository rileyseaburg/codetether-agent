//! Resources retained by a parallel stage until collection and finalization.
use super::parallel_result_types::ParallelStoryResult;
use super::*;
pub(in crate::ralph::ralph_loop) struct ParallelStage {
    pub(in crate::ralph::ralph_loop) handles: Vec<tokio::task::JoinHandle<ParallelStoryResult>>,
    pub(in crate::ralph::ralph_loop) provider: Arc<dyn Provider>,
    pub(in crate::ralph::ralph_loop) model: String,
    pub(in crate::ralph::ralph_loop) working_dir: PathBuf,
    pub(in crate::ralph::ralph_loop) _semaphore: Arc<tokio::sync::Semaphore>,
}
