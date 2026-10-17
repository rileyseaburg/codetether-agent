//! Decode TypeScript's synchronous diagnostic response without losing locations.

use crate::types::{DiagnosticInfo, PositionInfo, RangeInfo};
use serde::Deserialize;

/// One diagnostic returned by TypeScript's synchronous request commands.
///
/// # Examples
/// ```
/// let diagnostic: codetether_lsp_core::typescript_diagnostic::Diagnostic = serde_json::from_value(serde_json::json!({
///     "start": {"line": 1, "offset": 1}, "end": {"line": 1, "offset": 2},
///     "text": "example", "code": 1, "category": "error"
/// })).unwrap();
/// assert_eq!(diagnostic.into_lsp("file:///test.ts").range.start.line, 0);
/// ```
#[derive(Deserialize)]
pub struct Diagnostic {
    start: Position,
    end: Position,
    text: String,
    code: u32,
    category: String,
}

#[derive(Deserialize)]
struct Position {
    line: u32,
    offset: u32,
}

impl Position {
    fn into_lsp(self) -> PositionInfo {
        PositionInfo {
            line: self.line.saturating_sub(1),
            character: self.offset.saturating_sub(1),
        }
    }
}

impl Diagnostic {
    /// Convert one-based TypeScript positions into zero-based LSP positions.
    pub fn into_lsp(self, uri: &str) -> DiagnosticInfo {
        DiagnosticInfo {
            uri: uri.into(),
            range: RangeInfo {
                start: self.start.into_lsp(),
                end: self.end.into_lsp(),
            },
            severity: Some(self.category),
            code: Some(self.code.to_string()),
            source: Some("typescript".into()),
            message: self.text,
        }
    }
}
