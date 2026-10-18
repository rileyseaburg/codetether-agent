//! Attempt state and ordered recovery from provider completion failures.
//!
//! [recover] prioritizes context compaction, provider-requested delays and then
//! bounded upstream retries. [Host] supplies native derivation and failover;
//! [Attempt] retains counters and the derived history across attempts.
mod host;
mod interlude;
mod overflow;
mod recover;
mod retry_after;
mod state;
pub use host::Host;
pub use overflow::resolve as derivation;
pub use recover::recover;
pub use state::Attempt;

/// Request dispatch, success accounting and failure-recovery loop.
pub mod driver;
