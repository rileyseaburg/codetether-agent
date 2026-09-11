//! Executes proposed-content diagnostics through actual language servers.

use std::{path::PathBuf, sync::Arc};

use serde_json::Value;

use crate::lsp::{LspActionResult, LspManager};
use crate::tui::app::state::approval_queue::{ApprovalReport, ApprovalReportState};

pub(super) async fn inspect(
    manager: Arc<LspManager>,
    root: &PathBuf,
    tool: &str,
    arguments: &Value,
) -> ApprovalReport {
    let files = match crate::tool::proposed_content::contents(root, tool, arguments) {
        Ok(files) => files,
        Err(error) => return unavailable(error.to_string()),
    };
    let mut messages = Vec::new();
    for (path, content) in files {
        let client = match manager.get_client_for_file(&path).await {
            Ok(client) => client,
            Err(error) => return unavailable(error.to_string()),
        };
        match client.diagnostics_for_content(&path, &content).await {
            Ok(LspActionResult::Diagnostics { diagnostics }) => {
                messages.extend(diagnostics.into_iter().map(|item| {
                    format!(
                        "{}:{} {}: {}",
                        path.display(),
                        item.range.start.line + 1,
                        item.severity.unwrap_or_else(|| "diagnostic".into()),
                        item.message,
                    )
                }));
            }
            Ok(_) => return unavailable("language server returned an unexpected result".into()),
            Err(error) => return unavailable(error.to_string()),
        }
    }
    let state = if messages.is_empty() {
        ApprovalReportState::Clean
    } else {
        ApprovalReportState::Issues
    };
    messages.truncate(3);
    ApprovalReport { state, messages }
}

fn unavailable(message: String) -> ApprovalReport {
    ApprovalReport {
        state: ApprovalReportState::Unavailable,
        messages: vec![message],
    }
}
