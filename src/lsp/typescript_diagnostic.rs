//! Decode TypeScript's synchronous diagnostic response without losing locations.

use crate::lsp::types::{DiagnosticInfo, PositionInfo, RangeInfo};
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Diagnostic {
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
    pub(super) fn into_lsp(self, uri: &str) -> DiagnosticInfo {
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
