//! parse hover response operations for language server clients.

use super::*;

/// Decodes a hover protocol response into an action result.
///
/// # Arguments
/// * `response` - The server's JSON-RPC response, including any protocol error.
///
/// # Returns
/// The decoded action, an empty result for absent data, or a protocol error action.
///
/// # Errors
/// Returns an error when a non-null hover payload cannot be deserialized.
///
/// # Examples
/// ```
/// use codetether_lsp_core::{JsonRpcResponse, LspActionResult};
/// use codetether_lsp_core::response::parse_hover_response;
/// let response = JsonRpcResponse {
///     jsonrpc: "2.0".into(), id: 1, result: None, error: None,
/// };
/// let action = parse_hover_response(response).unwrap();
/// assert!(matches!(action, LspActionResult::Hover { contents, .. } if contents.is_empty()));
/// ```
pub fn parse_hover_response(response: JsonRpcResponse) -> Result<LspActionResult> {
    if let Some(error) = response.error {
        return Ok(LspActionResult::Error {
            message: error.message,
        });
    }

    let Some(result) = response.result else {
        return Ok(LspActionResult::Hover {
            contents: String::new(),
            range: None,
        });
    };

    if result.is_null() {
        return Ok(LspActionResult::Hover {
            contents: "No hover information available".to_string(),
            range: None,
        });
    }

    let hover: lsp_types::Hover = serde_json::from_value(result)?;

    let contents = match hover.contents {
        lsp_types::HoverContents::Scalar(markup) => match markup {
            lsp_types::MarkedString::String(s) => s,
            lsp_types::MarkedString::LanguageString(ls) => ls.value,
        },
        lsp_types::HoverContents::Array(markups) => markups
            .into_iter()
            .map(|m| match m {
                lsp_types::MarkedString::String(s) => s,
                lsp_types::MarkedString::LanguageString(ls) => ls.value,
            })
            .collect::<Vec<_>>()
            .join("\n\n"),
        lsp_types::HoverContents::Markup(markup) => markup.value,
    };

    Ok(LspActionResult::Hover {
        contents,
        range: hover.range.map(RangeInfo::from),
    })
}
