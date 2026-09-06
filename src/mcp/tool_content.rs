//! MCP tool content blocks with protocol-correct image MIME field spelling.
//!
//! [`ToolContent`] is shared by MCP clients and servers. Images retain their
//! base64 payload and serialize `mimeType`, as required by the MCP protocol.

use super::ResourceContents;
use serde::{Deserialize, Serialize};

/// Text, image, or embedded resource returned by an MCP tool.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::mcp::ToolContent;
/// let block = ToolContent::Image {
///     data: "eA==".into(), mime_type: "image/png".into(),
/// };
/// match &block {
///     ToolContent::Image { mime_type, .. } => assert_eq!(mime_type, "image/png"),
///     ToolContent::Text { .. } | ToolContent::Resource { .. } => unreachable!(),
/// }
/// assert_eq!(serde_json::to_value(block).unwrap()["mimeType"], "image/png");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolContent {
    /// Human-readable textual output.
    Text { text: String },
    /// Base64 image bytes and their MIME type (not a double-encoded data URL).
    Image {
        data: String,
        #[serde(rename = "mimeType", alias = "mime_type")]
        mime_type: String,
    },
    /// An embedded resource, preserved independently of image attachments.
    Resource { resource: ResourceContents },
}
