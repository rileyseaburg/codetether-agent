//! Effective trust-policy status storage, labels and terminal guidance.
//!
//! Call [set_from_config] when configuration is loaded. [current_status]
//! returns that snapshot; [trust_status_badge_span] and guidance functions
//! describe it for the terminal UI without changing approval decisions.
mod display;
mod guidance;
mod labels;
mod store;
mod summary;
pub use display::{approval_guidance, format_status, startup_summary, trust_status_badge_span};
pub use store::{current_status, set_from_config, set_status};
#[cfg(test)]
mod tests;
