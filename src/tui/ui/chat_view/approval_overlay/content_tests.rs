//! Approval explanations and diagnostics remain in the scrollable content.

use super::test_support::{QueueGuard, queue};
use crate::approval::test_env::lock_env;
use crate::tui::app::state::approval_queue::{self, ApprovalReport, ApprovalReportState};

#[test]
fn approval_multiline_reasons_and_all_diagnostics_are_preserved() {
    let _lock = lock_env();
    approval_queue::reset();
    let _guard = QueueGuard;
    let mut item = queue("command one\ncommand two");
    item.reason = "reason one\nreason two".into();
    item.justification = Some("explain one\nexplain two".into());
    item.report = ApprovalReport {
        state: ApprovalReportState::Issues,
        messages: (0..12).map(|i| format!("diagnostic {i}")).collect(),
    };
    let text: Vec<String> = super::super::approval_overlay_content::lines(&item)
        .iter()
        .map(ToString::to_string)
        .collect();
    for value in ["reason two", "explain two", "command two", "diagnostic 11"] {
        assert!(text.iter().any(|line| line == value), "missing {value}");
    }
}
