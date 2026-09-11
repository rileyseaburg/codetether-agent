//! Asynchronous real-LSP analysis of proposed file mutations.

use std::{path::PathBuf, sync::Arc};

use serde_json::{Value, json};

use crate::lsp::LspManager;
use crate::tui::app::state::{
    App,
    approval_queue::{self, ApprovalReport, ApprovalSnapshot},
};

#[path = "approval_analysis_run.rs"]
mod run;

pub(super) fn start(app: &mut App, item: &ApprovalSnapshot) {
    if !crate::tool::proposed_content::supported(&item.tool) {
        return;
    }
    let Some(arguments) = arguments(item) else {
        return;
    };
    let root = PathBuf::from(&app.state.cwd_display);
    let manager = app
        .state
        .editor_lsp
        .manager
        .get_or_insert_with(|| Arc::new(LspManager::new(Some(crate::lsp::path_to_uri(&root)))))
        .clone();
    let id = item.id.clone();
    let tool = item.tool.clone();
    approval_queue::set_report(&id, ApprovalReport::checking());
    tokio::spawn(async move {
        let report = run::inspect(manager, &root, &tool, &arguments).await;
        approval_queue::set_report(&id, report);
    });
}

/// Raw arguments, or a patch-only shape for requests from older runtimes.
fn arguments(item: &ApprovalSnapshot) -> Option<Value> {
    if let Some(arguments) = &item.arguments {
        return Some(arguments.clone());
    }
    let legacy_patch = matches!(item.tool.as_str(), "apply_patch" | "patch");
    legacy_patch
        .then(|| item.preview.as_ref())
        .flatten()
        .map(|patch| json!({"patch": patch}))
}
