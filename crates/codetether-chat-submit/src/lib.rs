//! Ordered chat input routing independent of terminal and session types.
//!
//! [submit] gives goal answers and approval feedback precedence, captures the
//! prompt, resolves managed-agent/mention routing, and finally chooses command,
//! steering or ordinary submission. Implement [Host] using native UI handlers.
mod intercept;
mod routing;
mod run;
mod submission;
pub use intercept::Intercept;
pub use routing::Routing;
pub use run::submit;
pub use submission::Submission;
/// Native operations required by the shared chat-routing sequence.
/// # Implementors
/// Any type implementing the three focused handler contracts is a host.
pub trait Host: Intercept + Routing + Submission {}
impl<T: Intercept + Routing + Submission> Host for T {}

/// Attachment capture, goal adoption and dispatch finalization.
pub mod finish;

/// Held-goal gating and provider dispatch preparation.
pub mod dispatch;

/// Owned prompt/session handoff with runtime rejection recovery.
pub mod handoff;
