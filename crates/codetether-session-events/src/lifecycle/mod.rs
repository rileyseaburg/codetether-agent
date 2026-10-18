//! Thinking, completion and restored-session lifecycle events.
//!
//! [handle_event] preserves lifecycle transition order. [Processing] supplies
//! worker/timing operations, [Completion] supplies completion cleanup, and
//! [Sync] retains ownership of the frontend's concrete session type.
mod completion;
mod processing;
mod run;
mod sync;
pub use completion::Completion;
pub use processing::Processing;
pub use run::handle_event;
pub use sync::{Session, Sync};
/// Combined frontend lifecycle operations.
/// # Implementors
/// Automatically implemented for types implementing all three lifecycle traits.
pub trait Host: Processing + Completion + Sync {}
impl<T: Processing + Completion + Sync> Host for T {}
