//! Standard worker session preparation and execution policy.
//! [execute] selects a provider, records delegated input, prepares tools and
//! runs a bounded session. [Host] binds native session and registry services.
mod execute;
mod execution;
mod source;
mod types;
pub use execute::execute;
pub use execution::Execution;
pub use source::{Registry, Source};
pub use types::{OutputCallback, PolicySessionResult, RunRequest};
/// Native services required for standard worker session execution.
/// # Implementors
/// Combine source/session services with native tool construction and execution.
pub trait Host: Source + Execution {}
impl<T: Source + Execution> Host for T {}

mod result;
mod run_selected;
