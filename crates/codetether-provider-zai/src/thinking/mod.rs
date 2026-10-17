//! Shared thinking policy for exact answers and short requests.
//!
//! The decision and JSON configuration functions share one instruction rule.
mod config;
mod decision;
mod instruction;
pub use config::thinking_config_for_request;
pub use decision::should_disable_thinking_for_request;
