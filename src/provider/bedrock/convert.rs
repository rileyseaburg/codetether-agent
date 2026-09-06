//! Convert the crate's generic [`Message`] format to the Bedrock Converse
//! API's JSON schema, and back-convert tool definitions.
//!
//! Bedrock Converse requires:
//! - A separate top-level `system` array for system prompts.
//! - Strict alternation of `user` / `assistant` messages.
//! - Assistant tool-use blocks with `toolUse` objects (`input` as an object).
//! - Tool results appear in the *next user* message as `toolResult` blocks.
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::provider::bedrock::{convert_messages, convert_tools};
//! use codetether_agent::provider::{ContentPart, Message, Role, ToolDefinition};
//! use serde_json::json;
//!
//! let msgs = vec![
//!     Message {
//!         role: Role::System,
//!         content: vec![ContentPart::Text { text: "You are helpful.".into() }],
//!     },
//!     Message {
//!         role: Role::User,
//!         content: vec![ContentPart::Text { text: "hi".into() }],
//!     },
//! ];
//! let (system, api_msgs) = convert_messages(&msgs);
//! assert_eq!(system.len(), 1);
//! assert_eq!(api_msgs.len(), 1);
//! assert_eq!(api_msgs[0]["role"], "user");
//!
//! let tools = vec![ToolDefinition {
//!     name: "echo".into(),
//!     description: "Echo text".into(),
//!     parameters: json!({"type":"object"}),
//! }];
//! let converted = convert_tools(&tools);
//! assert_eq!(converted[0]["toolSpec"]["name"], "echo");
//! ```

use crate::provider::{Message, Role};
use serde_json::Value;

mod assistant;
mod image;
#[cfg(test)]
mod image_test_support;
#[cfg(test)]
mod image_tests;
mod merge;
mod repair;
mod system;
mod tool;
mod tools;
mod user;
use assistant::append_assistant;
use system::append_system;
use tool::append_tool;
pub use tools::convert_tools;
use user::append_user;

/// Convert generic [`Message`]s to Bedrock Converse API format.
///
/// IMPORTANT: Bedrock requires strict role alternation (user/assistant).
/// Consecutive [`Role::Tool`] messages are merged into a single `"user"`
/// message so all `toolResult` blocks for a given assistant turn appear
/// together. Consecutive same-role messages are also merged to prevent
/// validation errors.
///
/// # Arguments
///
/// * `messages` — The crate-internal chat transcript to send.
///
/// # Returns
///
/// A tuple `(system_parts, api_messages)`:
/// - `system_parts`: objects suitable for the top-level `"system"` array.
/// - `api_messages`: objects suitable for the top-level `"messages"` array.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::bedrock::convert_messages;
/// use codetether_agent::provider::{ContentPart, Message, Role};
///
/// let msgs = vec![Message {
///     role: Role::User,
///     content: vec![ContentPart::Text { text: "hello".into() }],
/// }];
/// let (system, api_msgs) = convert_messages(&msgs);
/// assert!(system.is_empty());
/// assert_eq!(api_msgs.len(), 1);
/// assert_eq!(api_msgs[0]["content"][0]["text"], "hello");
/// ```
pub fn convert_messages(messages: &[Message]) -> (Vec<Value>, Vec<Value>) {
    let mut system_parts: Vec<Value> = Vec::new();
    let mut api_messages: Vec<Value> = Vec::new();

    for msg in messages {
        match msg.role {
            Role::System | Role::Developer => append_system(msg, &mut system_parts),
            Role::User => append_user(msg, &mut api_messages),
            Role::Assistant => append_assistant(msg, &mut api_messages),
            Role::Tool => append_tool(msg, &mut api_messages),
        }
    }

    repair::tool_exchanges(&mut api_messages);
    (system_parts, api_messages)
}
