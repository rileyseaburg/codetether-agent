//! parse document symbols response operations for language server clients.

use super::*;

/// Decodes a document symbols protocol response into an action result.
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
/// use codetether_lsp_core::response::parse_document_symbols_response;
/// let response = JsonRpcResponse {
///     jsonrpc: "2.0".into(), id: 1, result: None, error: None,
/// };
/// let action = parse_document_symbols_response(response).unwrap();
/// assert!(matches!(action, LspActionResult::DocumentSymbols { symbols } if symbols.is_empty()));
/// ```
pub fn parse_document_symbols_response(response: JsonRpcResponse) -> Result<LspActionResult> {
    if let Some(error) = response.error {
        return Ok(LspActionResult::Error {
            message: error.message,
        });
    }

    let Some(result) = response.result else {
        return Ok(LspActionResult::DocumentSymbols { symbols: vec![] });
    };

    if result.is_null() {
        return Ok(LspActionResult::DocumentSymbols { symbols: vec![] });
    }

    if let Ok(symbols) = serde_json::from_value::<Vec<lsp_types::DocumentSymbol>>(result.clone()) {
        return Ok(LspActionResult::DocumentSymbols {
            symbols: symbols.into_iter().map(SymbolInfo::from).collect(),
        });
    }

    if let Ok(symbols) = serde_json::from_value::<Vec<lsp_types::SymbolInformation>>(result) {
        return Ok(LspActionResult::DocumentSymbols {
            symbols: symbols.into_iter().map(SymbolInfo::from).collect(),
        });
    }

    Ok(LspActionResult::DocumentSymbols { symbols: vec![] })
}
