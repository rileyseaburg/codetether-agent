//! Borrowed run context separating cycle control from startup ownership.
//!
//! RunLoop borrows all live owners from the entrypoint. It handles one cycle
//! at a time and reports normal shutdown after loop completion.
use super::*;
mod cycle;
mod run;
mod summary;
pub(super) struct RunLoop<'a> {
    pub args: &'a ForageArgs,
    pub repo: &'a OkrRepository,
    pub moonshot_rubric: &'a MoonshotRubric,
    pub events: &'a events::RunEvents<'a>,
    pub observer: &'a mut BusHandle,
    pub s3_sync_handle: &'a mut Option<tokio::task::JoinHandle<Result<()>>>,
    pub require_s3: bool,
}
