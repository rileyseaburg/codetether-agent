//! Bedrock context-window and output-capacity estimates.
//!
//! Context estimates use provider-core; output estimates preserve the
//! Bedrock-specific model-family policy.
#[path = "estimates/context.rs"]
mod context;
#[path = "estimates/output.rs"]
mod output;
#[path = "estimates/remaining.rs"]
mod remaining;
pub use context::estimate_context_window;
pub use output::estimate_max_output;
