//! Post-agent result finalization and repository publication.
//!
//! [finalize_task_result] applies deadline reporting, publishes eligible
//! workspace changes, and updates result fields and progress. Callers provide
//! [FinalizationWorkspace] instead of an application session.
mod finalize;
mod publication;
mod workspace;
pub use finalize::finalize_task_result;
pub use workspace::FinalizationWorkspace;
