//! Native session views and context-completion interface.
//!
//! [State] exposes the existing agent/session values and prompt composition.
//! [Completion] invokes the native context builder and provider path.
mod completion;
mod state;
pub use completion::Completion;
pub use state::State;
/// Services required by [crate::execute].
/// # Implementors
/// Implement the native state and completion interfaces on an existing replay host.
pub trait Host: State + Completion {}
impl<T: State + Completion> Host for T {}
