//! FIFO queue access and mutation operations.

use crate::approval::LiveApprovalRequest;

use super::{ApprovalSnapshot, queue};

pub(crate) fn push(request: LiveApprovalRequest) -> ApprovalSnapshot {
    let snapshot = ApprovalSnapshot::from(request);
    let mut guard = queue().lock().expect("approval queue lock");
    guard.retain(|item| item.id != snapshot.id);
    guard.push_back(snapshot.clone());
    snapshot
}

pub(crate) fn active() -> Option<ApprovalSnapshot> {
    queue()
        .lock()
        .expect("approval queue lock")
        .front()
        .cloned()
}

pub(crate) fn active_id() -> Option<String> {
    active().map(|item| item.id)
}

pub(crate) fn len() -> usize {
    queue().lock().expect("approval queue lock").len()
}

pub(crate) fn resolve(id: &str) {
    queue()
        .lock()
        .expect("approval queue lock")
        .retain(|item| item.id != id);
}

#[cfg(test)]
pub(crate) fn reset() {
    queue().lock().expect("approval queue lock").clear();
}
