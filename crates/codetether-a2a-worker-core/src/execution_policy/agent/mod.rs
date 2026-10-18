//! Select swarm or standard worker execution and normalize its result.
//! [execute] uses the established agent classification and result mapping;
//! [Host] retains native execution configuration and session access.
mod host;
mod run;
pub use host::Host;
pub use run::execute;
/// Normalized task status, result, error and session identifier.
pub type AgentOutcome = (&'static str, Option<String>, Option<String>, Option<String>);
