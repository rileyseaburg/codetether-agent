//! Asynchronous real-LSP analysis of proposed patch contents.

use std::{path::PathBuf, sync::Arc};

use crate::lsp::LspManager;
use crate::tui::app::state::{
    App,
    approval_queue::{self, ApprovalReport, ApprovalSnapshot},
};

#[path = "approval_analysis_run.rs"]
mod run;

pub(super) fn start(app: &mut App, item: &ApprovalSnapshot) {
    if item.tool != "apply_patch" {
        return;
    }
    let Some(preview) = item.preview.clone() else {
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
    approval_queue::set_report(&id, ApprovalReport::checking());
    tokio::spawn(async move {
        let report = run::inspect(manager, &root, &preview).await;
        approval_queue::set_report(&id, report);
    });
}
