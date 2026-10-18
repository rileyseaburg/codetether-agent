//! Ordinary CLI session execution composed from workspace-owned services.
//!
//! Prepare an [Input], supply the native [Host], then call [execute]. Session
//! selection, policy, live events, resume and output retain their dedicated owners.
mod execute;
mod host;
mod input;
mod turn;
pub use execute::execute;
pub use host::{Host, Session};
pub use input::Input;
