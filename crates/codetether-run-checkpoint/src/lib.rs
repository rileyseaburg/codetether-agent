//! Resumable checkpoints extracted from an existing run transcript.
//!
//! [RunCheckpoint::from_session_messages] preserves run identity while scanning
//! assistant actions, tool errors and browser URLs. [auto_resume_prompt] renders
//! the existing continuation instructions. Persistence stays with the session.
mod checkpoint_assemble;
mod checkpoint_assistant;
mod checkpoint_build;
mod checkpoint_defaults;
mod checkpoint_state;
mod checkpoint_text;
mod checkpoint_tool;
mod checkpoint_update;
mod checkpoint_walk;
mod model;
mod prompt;
mod reason;
pub use model::RunCheckpoint;
pub use prompt::auto_resume_prompt;
pub use reason::CheckpointReason;

/// Checkpoint sidecars and native session metadata synchronization.
pub mod storage;
