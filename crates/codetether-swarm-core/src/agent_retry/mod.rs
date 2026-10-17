//! Retry policy for incomplete or failed sub-agent attempts.
//!
//! [run] applies bounded exponential backoff to a host [AgentAttempt].
//! [AgentLoopExit] distinguishes completion from step/deadline limits.
use anyhow::Result;
mod attempt;
mod backoff;
mod exit;
mod logging;
mod run;
pub use attempt::AgentAttempt;
pub use exit::AgentLoopExit;
pub use run::run;
/// Text, step/tool counts and terminal reason returned by one attempt.
pub type AgentOutcome = Result<(String, usize, usize, AgentLoopExit)>;
#[cfg(test)]
mod tests;
