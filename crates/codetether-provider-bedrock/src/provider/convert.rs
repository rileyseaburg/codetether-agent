//! Bedrock message and tool conversion compatibility exports.
//!
//! [convert_messages] normalizes transcript roles and tool exchanges;
//! [convert_tools] encodes provider-neutral tool definitions.
pub use codetether_bedrock_protocol::convert::{convert_messages, convert_tools};
