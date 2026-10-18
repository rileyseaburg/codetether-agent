//! Owned queue transfer for worker SSE responses.
use crate::http::HttpState;
use codetether_task_queue::KnativeTaskQueue;
/// Request state that can transfer its queue handle to the response stream.
/// # Implementors
/// Applications return the same queue exposed by [HttpState::queue].
pub trait WorkerState: HttpState {
    /// Consume the request state and return its queue without replacing it.
    fn into_queue(self) -> KnativeTaskQueue;
}
