//! DiagnosticInfo text and approval report construction.
use codetether_approval_queue::{ApprovalReport, ApprovalReportState};
use codetether_lsp_core::DiagnosticInfo;
use std::path::Path;
pub(crate) fn diagnostic(path: &Path, item: DiagnosticInfo) -> String {
    format!(
        "{}:{} {}: {}",
        path.display(),
        item.range.start.line + 1,
        item.severity.unwrap_or_else(|| "diagnostic".into()),
        item.message
    )
}
pub(crate) fn finish(mut messages: Vec<String>) -> ApprovalReport {
    let state = if messages.is_empty() {
        ApprovalReportState::Clean
    } else {
        ApprovalReportState::Issues
    };
    messages.truncate(3);
    ApprovalReport { state, messages }
}

pub(crate) fn unavailable(message: String) -> ApprovalReport {
    ApprovalReport {
        state: ApprovalReportState::Unavailable,
        messages: vec![message],
    }
}
