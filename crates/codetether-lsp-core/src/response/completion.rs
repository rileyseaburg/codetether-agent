//! parse completion response operations for language server clients.

use super::*;

/// Decodes a completion protocol response into an action result.
///
/// # Arguments
/// * `response` - The server's JSON-RPC response, including any protocol error.
///
/// # Returns
/// The decoded action, an empty result for absent data, or a protocol error action.
///
/// # Errors
/// This decoder currently represents protocol errors as successful error actions.
///
/// # Examples
/// ```
/// use codetether_lsp_core::{JsonRpcResponse, LspActionResult};
/// use codetether_lsp_core::response::parse_completion_response;
/// let response = JsonRpcResponse {
///     jsonrpc: "2.0".into(), id: 1, result: None, error: None,
/// };
/// let action = parse_completion_response(response).unwrap();
/// assert!(matches!(action, LspActionResult::Completion { items } if items.is_empty()));
/// ```
pub fn parse_completion_response(response: JsonRpcResponse) -> Result<LspActionResult> {
    if let Some(error) = response.error {
        return Ok(LspActionResult::Error {
            message: error.message,
        });
    }

    let Some(result) = response.result else {
        return Ok(LspActionResult::Completion { items: vec![] });
    };

    if result.is_null() {
        return Ok(LspActionResult::Completion { items: vec![] });
    }

    if let Ok(list) = serde_json::from_value::<lsp_types::CompletionList>(result.clone()) {
        return Ok(LspActionResult::Completion {
            items: list
                .items
                .into_iter()
                .map(CompletionItemInfo::from)
                .collect(),
        });
    }

    if let Ok(items) = serde_json::from_value::<Vec<lsp_types::CompletionItem>>(result) {
        return Ok(LspActionResult::Completion {
            items: items.into_iter().map(CompletionItemInfo::from).collect(),
        });
    }

    Ok(LspActionResult::Completion { items: vec![] })
}
