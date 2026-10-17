//! Render an agent's tool catalog as a training system message.
//!
//! The emitted text mirrors the `<tools>` block that inference-time chat
//! templates inject, so exported transcripts train the model on the same
//! prompt shape it will actually receive.

use crate::tool_catalog::{ToolSchema, to_function_specs};

/// Build the system-message content describing available tools.
///
/// # Examples
///
/// ```rust
/// use codetether_bus_core::tool_catalog::ToolSchema;
/// use codetether_bus_core::training::catalog::catalog_text;
///
/// let text = catalog_text(&[ToolSchema {
///     name: "read".into(),
///     description: "Read a file".into(),
///     parameters: serde_json::json!({"type": "object"}),
/// }]);
/// assert!(text.starts_with("<tools>"));
/// assert!(text.contains("\"read\""));
/// ```
pub fn catalog_text(tools: &[ToolSchema]) -> String {
    let lines: Vec<String> = to_function_specs(tools)
        .iter()
        .map(|spec| serde_json::to_string(spec).unwrap_or_default())
        .collect();
    format!("<tools>\n{}\n</tools>", lines.join("\n"))
}
