//! Resources retained by a parallel stage until collection and finalization.
use super::*;
pub(crate) struct ParallelStage {
    pub(crate) handles: Vec<tokio::task::JoinHandle<ParallelStoryResult>>,
    pub(crate) provider: Arc<dyn Provider>,
    pub(crate) model: String,
    pub(crate) working_dir: PathBuf,
    pub(crate) _semaphore: Arc<tokio::sync::Semaphore>,
}
