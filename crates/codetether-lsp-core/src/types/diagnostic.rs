//! LSP diagnostic definitions and helpers.

use super::*;
use lsp_types::DiagnosticSeverity;

/// Simplified diagnostic info for tool output and proactive session context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticInfo {
    pub uri: String,
    pub range: RangeInfo,
    pub severity: Option<String>,
    pub code: Option<String>,
    pub source: Option<String>,
    pub message: String,
}

impl DiagnosticInfo {
    pub fn severity_rank(&self) -> u8 {
        match self.severity.as_deref() {
            Some("error") => 1,
            Some("warning") => 2,
            Some("information") => 3,
            Some("hint") => 4,
            _ => 5,
        }
    }
}

impl From<(String, lsp_types::Diagnostic)> for DiagnosticInfo {
    fn from((uri, diagnostic): (String, lsp_types::Diagnostic)) -> Self {
        let severity = diagnostic.severity.map(|severity| match severity {
            DiagnosticSeverity::ERROR => "error".to_string(),
            DiagnosticSeverity::WARNING => "warning".to_string(),
            DiagnosticSeverity::INFORMATION => "information".to_string(),
            DiagnosticSeverity::HINT => "hint".to_string(),
            _ => "unknown".to_string(),
        });

        let code = diagnostic.code.map(|code| match code {
            lsp_types::NumberOrString::Number(n) => n.to_string(),
            lsp_types::NumberOrString::String(s) => s,
        });

        Self {
            uri,
            range: RangeInfo::from(diagnostic.range),
            severity,
            code,
            source: diagnostic.source,
            message: diagnostic.message,
        }
    }
}
