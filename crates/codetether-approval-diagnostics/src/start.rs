//! Lazy manager initialization and background approval diagnostics.
use crate::arguments;
use codetether_approval_queue::{self as approval_queue, ApprovalReport, ApprovalSnapshot};
use codetether_lsp_core::manager::{LspManager, ManagedClient};
use std::{path::PathBuf, sync::Arc};
/// Schedule diagnostics for a supported approval request.
/// # Arguments
/// * `manager_slot` — Cached manager retaining the application's client identity.
/// * `cwd` — Workspace root for the proposed content.
/// * `item` — Request snapshot to inspect and annotate.
/// # Returns
/// Starts a background task, or returns immediately for unsupported/missing inputs.
/// # Panics
/// Panics when called outside a Tokio runtime for an inspectable request.
/// # Examples
/// ```no_run
/// # fn example<C: codetether_lsp_core::manager::ManagedClient>(manager: &mut Option<std::sync::Arc<codetether_lsp_core::manager::LspManager<C>>>, item: &codetether_approval_queue::ApprovalSnapshot) {
/// codetether_approval_diagnostics::start(manager, ".", item);
/// # }
/// ```
pub fn start<C: ManagedClient>(
    manager_slot: &mut Option<Arc<LspManager<C>>>,
    cwd: &str,
    item: &ApprovalSnapshot,
) {
    if !codetether_proposed_content::supported(&item.tool) {
        return;
    }
    let Some(arguments) = arguments(item) else {
        return;
    };
    let root = PathBuf::from(cwd);
    let manager = manager_slot
        .get_or_insert_with(|| {
            Arc::new(LspManager::new(Some(codetether_lsp_core::path_to_uri(
                &root,
            ))))
        })
        .clone();
    let id = item.id.clone();
    let tool = item.tool.clone();
    approval_queue::set_report(&id, ApprovalReport::checking());
    tokio::spawn(async move {
        let report = crate::inspect(manager, &root, &tool, &arguments).await;
        approval_queue::set_report(&id, report);
    });
}
