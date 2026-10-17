use super::*;
/// Normalize a handler result for the wire and bus publication.
///
/// # Arguments
/// `result` is the result of executing the selected tool handler.
/// # Returns
/// Serialized content, joined text output and the success flag.
/// # Errors
/// Response serialization errors; handler failures become error content.
///
/// # Examples
/// ```
/// use codetether_mcp_core::{server_protocol::normalize_tool_result, types::CallToolResult};
/// let (_, text, success) = normalize_tool_result(Ok(CallToolResult {
///     content: vec![], is_error: false,
/// })).unwrap();
/// assert!(success);
/// assert!(text.is_empty());
/// ```
pub fn normalize_tool_result(
    result: anyhow::Result<CallToolResult>,
) -> Result<(Value, String, bool), JsonRpcError> {
    Ok(match result {
        Ok(result) => {
            let text = result
                .content
                .iter()
                .filter_map(|c| match c {
                    ToolContent::Text { text } => Some(text.as_str()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("\n");
            let is_err = result.is_error;
            let val = serde_json::to_value(result)
                .map_err(|e| JsonRpcError::internal_error(e.to_string()))?;
            (val, text, !is_err)
        }
        Err(e) => {
            let err_text = e.to_string();
            let result = CallToolResult {
                content: vec![ToolContent::Text {
                    text: err_text.clone(),
                }],
                is_error: true,
            };
            let val = serde_json::to_value(result)
                .map_err(|e| JsonRpcError::internal_error(e.to_string()))?;
            (val, err_text, false)
        }
    })
}
