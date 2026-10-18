//! Native service boundaries for sequential story execution.
//!
//! [State] exposes live state, [Prompts] builds story context, [Reporting]
//! delivers progress and events, and [Execution] performs policy-gated effects.
mod execution;
mod prompts;
mod reporting;
mod state;
pub use execution::Execution;
pub use prompts::Prompts;
pub use reporting::Reporting;
pub use state::State;
/// Combined services required by [crate::run].
/// # Implementors
/// Implement each focused service trait on the native Ralph runtime.
pub trait Host: State + Prompts + Reporting + Execution {}
impl<T: State + Prompts + Reporting + Execution> Host for T {}
