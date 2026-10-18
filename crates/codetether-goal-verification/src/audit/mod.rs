//! Audited verification and rejection-cap escalation.
//!
//! Implement [Host] with session-path and revision-checked runtime operations,
//! then call [check] to verify, record the verdict and return any refusal.
mod check;
mod escalate;
mod host;
mod reject;
pub use check::check;
pub use host::Host;
