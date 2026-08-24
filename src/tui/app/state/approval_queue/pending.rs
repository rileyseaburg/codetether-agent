//! Removal of requests decided by another approval client.

use super::ApprovalSnapshot;
use std::collections::VecDeque;

pub(super) fn retain(queue: &mut VecDeque<ApprovalSnapshot>) {
    let store = match crate::approval::ApprovalStore::open_default() {
        Ok(store) => store,
        Err(error) => {
            tracing::error!(%error, "Approval store unavailable; clearing stale queue");
            queue.clear();
            return;
        }
    };
    queue.retain(|item| match store.decision(&item.id) {
        Ok(None) => true,
        Ok(Some(_)) => false,
        Err(error) => {
            tracing::error!(approval_id = %item.id, %error, "Approval store unreadable; pruning request");
            false
        }
    });
}

#[cfg(test)]
#[path = "pending_tests.rs"]
mod tests;
