//! Compare the committed receipt and terminal event with the winning release.

use super::{super::ReleaseRequest, events};
use crate::{bus::BusEnvelope, server::KnativeTask};

pub(super) fn receipt_matches(task: KnativeTask, status: &str, receipt: &ReleaseRequest) {
    assert_eq!(task.status, status);
    assert_eq!(task.completion.result, receipt.result);
    assert_eq!(task.completion.error, receipt.error);
    assert_eq!(task.completion.session_id, receipt.session_id);
    assert_eq!(task.completion.diagnostics, receipt.diagnostics);
}

pub(super) fn event_matches(event: BusEnvelope, status: &str, receipt: &ReleaseRequest) {
    let success = status == "completed";
    let message = if success {
        receipt.result.clone()
    } else {
        receipt
            .error
            .as_ref()
            .map(|error| format!("Error: {error}"))
    };
    events::assert_terminal_event_message(event, success, message.as_deref());
}
