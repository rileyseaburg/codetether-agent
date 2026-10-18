//! Render shared LSP diagnostics with stable relative paths and positions.
use super::paths::relative_display_path;
use std::path::{Path, PathBuf};

pub(super) fn render_diagnostics(
    workspace_dir: &Path,
    diagnostics: &[codetether_lsp_core::DiagnosticInfo],
) -> Vec<String> {
    diagnostics
        .iter()
        .map(|diagnostic| {
            let path = diagnostic
                .uri
                .strip_prefix("file://")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(&diagnostic.uri));
            let path = relative_display_path(workspace_dir, &path);
            let severity = diagnostic.severity.as_deref().unwrap_or("unknown");
            let source = diagnostic.source.as_deref().unwrap_or("lsp");
            let code = diagnostic
                .code
                .as_ref()
                .map(|code| format!(" ({code})"))
                .unwrap_or_default();
            format!(
                "[{severity}] {path}:{}:{} [{source}]{} {}",
                diagnostic.range.start.line + 1,
                diagnostic.range.start.character + 1,
                code,
                diagnostic.message.replace('\n', " ")
            )
        })
        .collect()
}
