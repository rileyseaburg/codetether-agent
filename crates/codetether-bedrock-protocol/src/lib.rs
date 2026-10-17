//! Bedrock request/response conversion, transcript repair, and model policy.
//!
//! [convert_messages] encodes provider-neutral messages and repairs tool
//! exchanges. [audit::enforce] validates and repairs final request JSON.
//! [conversation] handles model-specific final-turn compatibility.
//! [build_converse_body] applies inference and cache policy; [runtime_config]
//! shares live model overrides with calling applications.
//! [parse_converse_response] and [native::response] decode buffered replies;
//! [reasoning_audit] retains signatures through the shared audit service.
//! [eventstream] and [stream] decode incremental binary streaming responses.
pub mod audit;
pub mod conversation;
pub mod convert;
pub use convert::{convert_messages, convert_tools};
pub mod body;
pub mod estimates;
pub mod output_budget;
pub mod runtime_config;
pub use body::build_converse_body;
pub use estimates::{estimate_context_window, estimate_max_output};
pub mod empty_guard;
pub mod native;
pub mod reasoning;
pub mod reasoning_audit;
pub mod response;
pub use response::{BedrockError, parse_converse_response};
pub mod eventstream;
pub mod stream;
