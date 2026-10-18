//! Bounded reviewer execution and installed reviewer state for approvals.
//!
//! Call [review] with an application's [Host] to resolve providers and run its
//! read-only agent. [runtime::RuntimeStore] keeps reviewer configuration and
//! provider handles; verdicts never decide the user's approval.
mod host;
mod request;
mod run;
pub mod runtime;
pub use codetether_approval_core::review::{ReviewSubject, ReviewVerdict};
pub use host::Host;
pub use request::Run;
pub use run::review;

/// Background dispatch for queued tool approvals.
pub mod dispatch;
