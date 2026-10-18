//! Worker claim, completion and event-stream HTTP handlers.
//!
//! Use the shared [HttpState](crate::http::HttpState) for queue/bus access.
//! Implement [WorkerState] to transfer queue ownership into a long-lived SSE
//! stream after taking its initial snapshot. Applications retain route assembly.
mod claim;
mod release;
mod state;
mod stream;
pub use claim::worker_task_claim;
pub use release::worker_task_release;
pub use state::WorkerState;
pub use stream::{StreamQuery, worker_task_stream};
