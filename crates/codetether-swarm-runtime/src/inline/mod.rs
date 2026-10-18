//! Inline worker execution with concurrent event forwarding.
//!
//! [run] owns the executor future and never detaches it into a task. Implement
//! [Executor] using native resources; [emit_start] formats startup feedback.
mod executor;
mod run;
mod start;
pub use executor::Executor;
pub use run::run;
pub use start::emit_start;
/// Shared optional output sink for worker-facing lifecycle messages.
pub type Output = std::sync::Arc<dyn Fn(String) + Send + Sync + 'static>;
