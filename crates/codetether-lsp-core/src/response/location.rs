//! parse location response operations for language server clients.

use super::*;

/// Decodes a location protocol response into an action result.
///
/// # Arguments
/// * `response` - The server's JSON-RPC response, including any protocol error.
/// * `_operation` - Retained operation label; decoding does not branch on it.
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
/// use codetether_lsp_core::response::parse_location_response;
/// let response = JsonRpcResponse {
///     jsonrpc: "2.0".into(), id: 1, result: None, error: None,
/// };
/// let action = parse_location_response(response, "definition").unwrap();
/// assert!(matches!(action, LspActionResult::Definition { locations } if locations.is_empty()));
/// ```
pub fn parse_location_response(
    response: JsonRpcResponse,
    _operation: &str,
) -> Result<LspActionResult> {
    if let Some(error) = response.error {
        return Ok(LspActionResult::Error {
            message: error.message,
        });
    }

    let Some(result) = response.result else {
        return Ok(LspActionResult::Definition { locations: vec![] });
    };

    if let Ok(loc) = serde_json::from_value::<lsp_types::Location>(result.clone()) {
        return Ok(LspActionResult::Definition {
            locations: vec![LocationInfo::from(loc)],
        });
    }

    if let Ok(locs) = serde_json::from_value::<Vec<lsp_types::Location>>(result.clone()) {
        return Ok(LspActionResult::Definition {
            locations: locs.into_iter().map(LocationInfo::from).collect(),
        });
    }

    if let Ok(links) = serde_json::from_value::<Vec<lsp_types::LocationLink>>(result) {
        return Ok(LspActionResult::Definition {
            locations: links
                .into_iter()
                .map(|link| LocationInfo {
                    uri: link.target_uri.to_string(),
                    range: RangeInfo::from(link.target_selection_range),
                })
                .collect(),
        });
    }

    Ok(LspActionResult::Definition { locations: vec![] })
}
