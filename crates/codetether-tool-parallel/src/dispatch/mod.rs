//! Ordered execution of planned tool batches with sequential fallback.
//!
//! [run] consumes [crate::plan] barriers, tries parallel read groups, and falls
//! back to ordered calls when needed. [Host] supplies actual execution.
mod host;
mod run;
mod sequential;
pub use host::Host;
pub use run::run;
/// Provider call ID, tool name and raw JSON arguments before normalization.
pub type ToolCall = (String, String, serde_json::Value);
