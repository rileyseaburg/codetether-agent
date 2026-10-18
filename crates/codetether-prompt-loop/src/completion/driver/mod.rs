//! Completion-attempt loop with native request and effect boundaries.
//!
//! [complete] prepares and dispatches each request, records a successful
//! response, or persists failed-stream checkpoints before attempting recovery.
//! Implement [Requests] and [Effects] to supply native session services.
mod effects;
mod requests;
mod run;
pub use effects::Effects;
pub use requests::Requests;
pub use run::complete;
/// Services needed to execute a provider completion loop.
/// # Implementors
/// Any sendable host implementing request services and completion effects.
pub trait Host: Requests + Effects + Send {}
impl<T: Requests + Effects + Send> Host for T {}
