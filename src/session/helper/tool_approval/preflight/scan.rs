//! Inspect proposed files within one shared interactive preflight budget.

use crate::{
    lsp::{LspManager, detect_language_from_path},
    tool::ToolResult,
};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::time::{Duration, Instant};

pub(super) async fn run(
    workspace: &Path,
    tool: &str,
    files: Vec<(PathBuf, String)>,
    manager: Arc<LspManager>,
) -> (Option<ToolResult>, Vec<String>) {
    let deadline = Instant::now() + Duration::from_secs(5);
    let mut issues = Vec::new();
    let mut warnings = Vec::new();
    for (path, content) in files {
        let Some(language) = detect_language_from_path(path.to_string_lossy().as_ref()) else {
            continue;
        };
        if let Some(reason) = super::cooldown::reason(workspace, language) {
            warnings.push(format!("{}: {reason}", path.display()));
            continue;
        }
        if Instant::now() >= deadline {
            warnings.push(format!(
                "{}: shared LSP preflight budget exhausted",
                path.display()
            ));
            continue;
        }
        match super::check::run(&manager, workspace, &path, &content, deadline).await {
            Ok(result) => issues.extend(super::diagnostics::errors(workspace, &path, result)),
            Err(error) => {
                let reason = format!("{}: {error:#}", path.display());
                tracing::warn!(path = %path.display(), %error, "Pre-approval LSP unavailable");
                warnings.push(reason);
            }
        }
    }
    (
        (!issues.is_empty()).then(|| super::report::diagnostics(tool, issues)),
        warnings,
    )
}
