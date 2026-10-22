//! parse workspace symbols response operations for language server clients.

use super::*;

/// Decodes a workspace symbols protocol response into an action result.
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
/// use codetether_lsp_core::response::parse_workspace_symbols_response;
/// let response = JsonRpcResponse {
///     jsonrpc: "2.0".into(), id: 1, result: None, error: None,
/// };
/// let action = parse_workspace_symbols_response(response).unwrap();
/// assert!(matches!(action, LspActionResult::WorkspaceSymbols { symbols } if symbols.is_empty()));
/// ```
pub fn parse_workspace_symbols_response(response: JsonRpcResponse) -> Result<LspActionResult> {
    if let Some(error) = response.error {
        return Ok(LspActionResult::Error {
            message: error.message,
        });
    }

    let Some(result) = response.result else {
        return Ok(LspActionResult::WorkspaceSymbols { symbols: vec![] });
    };

    if result.is_null() {
        return Ok(LspActionResult::WorkspaceSymbols { symbols: vec![] });
    }

    if let Ok(symbols) = serde_json::from_value::<Vec<lsp_types::SymbolInformation>>(result.clone())
    {
        return Ok(LspActionResult::WorkspaceSymbols {
            symbols: symbols.into_iter().map(SymbolInfo::from).collect(),
        });
    }

    if let Ok(symbols) = serde_json::from_value::<Vec<lsp_types::WorkspaceSymbol>>(result) {
        return Ok(LspActionResult::WorkspaceSymbols {
            symbols: symbols
                .into_iter()
                .map(|s| {
                    let (uri, range) = match s.location {
                        lsp_types::OneOf::Left(loc) => {
                            (loc.uri.to_string(), Some(RangeInfo::from(loc.range)))
                        }
                        lsp_types::OneOf::Right(wl) => (wl.uri.to_string(), None),
                    };
                    SymbolInfo {
                        name: s.name,
                        kind: format!("{:?}", s.kind),
                        detail: None,
                        uri: Some(uri),
                        range,
                        container_name: s.container_name,
                    }
                })
                .collect(),
        });
    }

    Ok(LspActionResult::WorkspaceSymbols { symbols: vec![] })
}
