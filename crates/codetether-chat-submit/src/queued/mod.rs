//! Deferred chat submission after an active turn has finished.
//!
//! [drain] delivers pending goal input before checking whether a queued prompt
//! can run. In-memory gates precede the native held-goal check and queue take.
mod host;
mod run;
pub use host::{Commands, Input, Queue, State};
pub use run::drain;
/// Combined native operations used to drain one queued prompt.
/// # Implementors
/// Implement the command, queue and input contracts for native UI state.
pub trait Host: Commands + Queue + Input {}
impl<T: Commands + Queue + Input> Host for T {}
