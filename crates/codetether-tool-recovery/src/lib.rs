//! Recovery for missing or truncated provider tool calls.
//!
//! [salvage_prose_tool_call] recovers narrated calls only for the configured
//! provider cases and when no native or truncated calls exist. [error_content]
//! and [retry_prompt] guide smaller retries after output-token truncation.
//! This crate creates call data and messages; it never executes commands.
mod bash;
mod error;
mod prose;
mod retry;
mod rules;
mod salvage;
pub use bash::extract_bash_command;
pub use error::error_content;
pub use prose::extract_tool_call_from_prose;
pub use retry::retry_prompt;
pub use salvage::salvage_prose_tool_call;
#[cfg(test)]
mod tests;
