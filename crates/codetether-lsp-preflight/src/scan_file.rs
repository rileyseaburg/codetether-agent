//! Inspect one proposed file, preserving shared deadlines and language backoff.
use codetether_lsp_core::detect_language_from_path;
use codetether_lsp_core::manager::{LspManager, ManagedClient};
use std::{path::Path, sync::Arc};
use tokio::time::Instant;

pub(super) async fn inspect<C: ManagedClient>(
    workspace: &Path,
    path: &Path,
    content: &str,
    manager: &Arc<LspManager<C>>,
    deadline: Instant,
) -> (Vec<String>, Option<String>) {
    let Some(language) = detect_language_from_path(path.to_string_lossy().as_ref()) else {
        return (Vec::new(), None);
    };
    if let Some(reason) = super::cooldown::reason(workspace, language) {
        return (Vec::new(), Some(format!("{}: {reason}", path.display())));
    }
    if Instant::now() >= deadline {
        return (
            Vec::new(),
            Some(format!(
                "{}: shared LSP preflight budget exhausted",
                path.display()
            )),
        );
    }
    match super::check::run(manager, workspace, path, content, deadline).await {
        Ok(result) => (super::diagnostics::errors(workspace, path, result), None),
        Err(error) => {
            let reason = format!("{}: {error:#}", path.display());
            tracing::warn!(target: "codetether_agent::session::helper::tool_approval::preflight::scan", path = %path.display(), %error, "Pre-approval LSP unavailable");
            (Vec::new(), Some(reason))
        }
    }
}
