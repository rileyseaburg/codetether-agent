//! Filter and render error-severity language-server diagnostics.

use std::path::Path;

use crate::lsp::{DiagnosticInfo, LspActionResult};

pub(super) fn errors(workspace: &Path, path: &Path, result: LspActionResult) -> Vec<String> {
    let LspActionResult::Diagnostics { diagnostics } = result else {
        return Vec::new();
    };
    diagnostics
        .into_iter()
        .filter(is_error)
        .map(|item| render(workspace, path, &item))
        .collect()
}

fn is_error(item: &DiagnosticInfo) -> bool {
    item.severity.as_deref() == Some("error")
}

fn render(workspace: &Path, path: &Path, item: &DiagnosticInfo) -> String {
    let display = path.strip_prefix(workspace).unwrap_or(path).display();
    let line = item.range.start.line + 1;
    let column = item.range.start.character + 1;
    let code = item
        .code
        .as_deref()
        .map(|value| format!(" [{value}]"))
        .unwrap_or_default();
    format!("{display}:{line}:{column} error{code}: {}", item.message)
}
