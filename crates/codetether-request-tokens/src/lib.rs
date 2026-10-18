//! Estimate request token costs from shared provider types.
//!
//! [estimate_tokens_for_part] handles text, tools, reasoning and image parts.
//! [estimate_request_tokens] combines the system prompt, transcript and tool
//! definitions using the existing RLM character-based estimator.
mod messages;
mod part;
mod request;
mod thinking;
mod tools;
pub use messages::estimate_tokens_for_messages;
pub use part::estimate_tokens_for_part;
pub use request::estimate_request_tokens;
pub use tools::estimate_tokens_for_tools;
