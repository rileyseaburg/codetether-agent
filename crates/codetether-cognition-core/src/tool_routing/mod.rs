//! FunctionGemma prompt construction and output interpretation.
//!
//! [prioritize] and [build_functiongemma_prompt] prepare a request. Decode
//! results with [parse_functiongemma_response], supplying a host warning hook.
//! Inference, feature-specific setup and response mutation belong to the host.
mod identity_prompt;
mod inspect;
mod parse;
mod parse_blocks;
mod parsed_call;
mod prioritize;
mod prompt;
mod prompt_tools;
pub use identity_prompt::with_identity;
pub use inspect::{assistant_text, has_tool_calls};
pub use parse::parse_functiongemma_response;
pub use parse_blocks::blocks;
pub use parsed_call::ParsedToolCall;
pub use prioritize::prioritize;
pub use prompt::build_functiongemma_prompt;
pub use prompt_tools::{tool_lines, tools_json};

mod config;
mod config_env;
/// Configuration for the compiled-out router adapter.
pub mod disabled_config;
mod rewrite;
mod router_config;
pub use config::ToolRouterConfig;
pub use rewrite::rewrite_response;
pub use router_config::thinker_config;
