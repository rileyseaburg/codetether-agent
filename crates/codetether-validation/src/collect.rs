//! Collect ordered per-file LSP, linter and native ESLint diagnostics.
use super::{diagnostics::render_diagnostics, paths::relative_display_path};
use crate::Host;
use codetether_lsp_core::{LspActionResult, manager::LspManager};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
pub(super) async fn run<H: Host>(
    workspace_dir: &Path,
    existing_files: Vec<PathBuf>,
    manager: &LspManager<H::Client>,
    host: &H,
) -> (BTreeMap<String, Vec<String>>, usize) {
    let mut issues_by_file = BTreeMap::new();
    let mut issue_count = 0usize;

    for path in existing_files {
        let mut rendered = Vec::new();
        let client = manager.get_client_for_file(&path).await.ok();
        if let Some(client) = client
            && let Ok(LspActionResult::Diagnostics { diagnostics }) =
                client.diagnostics(&path).await
        {
            rendered.extend(render_diagnostics(workspace_dir, &diagnostics));
        }

        let linter_diagnostics = manager.linter_diagnostics(&path).await;
        rendered.extend(render_diagnostics(workspace_dir, &linter_diagnostics));
        rendered.extend(host.eslint(workspace_dir, &path).await);

        rendered.sort();
        rendered.dedup();

        if !rendered.is_empty() {
            issue_count += rendered.len();
            issues_by_file.insert(relative_display_path(workspace_dir, &path), rendered);
        }
    }

    (issues_by_file, issue_count)
}
