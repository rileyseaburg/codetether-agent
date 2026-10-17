//! Format diagnostics results for tool output.
use codetether_lsp_core::DiagnosticInfo;
pub(super) fn format(diagnostics: Vec<DiagnosticInfo>) -> String {
    if diagnostics.is_empty() {
        "No diagnostics found".to_string()
    } else {
        let mut out = format!("Diagnostics ({})\n\n", diagnostics.len());
        for diagnostic in diagnostics {
            out.push_str(&format!(
                "  [{}] {}:{}:{}",
                diagnostic.severity.as_deref().unwrap_or("unknown"),
                diagnostic.uri.trim_start_matches("file://"),
                diagnostic.range.start.line + 1,
                diagnostic.range.start.character + 1,
            ));
            if let Some(source) = diagnostic.source {
                out.push_str(&format!(" [{source}]"));
            }
            if let Some(code) = diagnostic.code {
                out.push_str(&format!(" ({code})"));
            }
            out.push_str(&format!("\n    {}\n", diagnostic.message));
        }
        out
    }
}
