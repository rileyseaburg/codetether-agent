//! State/reporting access and native scheduling operations.
//!
//! [State] lends the existing run data and event sink; [Execution] selects
//! the native checkout and sequential/parallel execution services.
mod execution;
mod state;
pub use execution::Execution;
pub use state::State;
/// Services required by [crate::run].
/// # Implementors
/// Implement the focused state and execution interfaces on a native runtime.
pub trait Host: State + Execution {}
impl<T: State + Execution> Host for T {}
