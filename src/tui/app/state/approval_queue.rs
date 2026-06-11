//! Pending live approval queue for the TUI.

use std::collections::VecDeque;
use std::sync::{Mutex, OnceLock};

use crate::approval::LiveApprovalRequest;

mod snapshot;
pub(crate) use snapshot::ApprovalSnapshot;

static QUEUE: OnceLock<Mutex<VecDeque<ApprovalSnapshot>>> = OnceLock::new();

fn queue() -> &'static Mutex<VecDeque<ApprovalSnapshot>> {
    QUEUE.get_or_init(|| Mutex::new(VecDeque::new()))
}

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
