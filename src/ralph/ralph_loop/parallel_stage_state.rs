//! Resources retained by a parallel stage until collection and finalization.
use super::parallel_result_types::ParallelStoryResult;
use super::*;
pub(super) struct ParallelStage {
    pub(super) handles: Vec<tokio::task::JoinHandle<ParallelStoryResult>>,
    pub(super) provider: Arc<dyn Provider>,
    pub(super) model: String,
    pub(super) working_dir: PathBuf,
    pub(super) _semaphore: Arc<tokio::sync::Semaphore>,
}
