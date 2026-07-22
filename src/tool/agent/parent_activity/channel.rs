//! Bounded pending-activity queue and notification primitive.

use std::collections::VecDeque;
use std::sync::Mutex;
use tokio::sync::Notify;

const MAX_PENDING: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Activity {
    Mailbox,
    Steered(u64),
}

#[derive(Default)]
pub(super) struct Channel {
    pending: Mutex<VecDeque<Activity>>,
    pub(super) notify: Notify,
}

impl Channel {
    pub(super) fn push(&self, activity: Activity) {
        let mut pending = self.pending.lock().expect("parent activity lock poisoned");
        if pending.len() == MAX_PENDING {
            pending.pop_front();
        }
        pending.push_back(activity);
        drop(pending);
        self.notify.notify_waiters();
        self.notify.notify_one();
    }

    pub(super) fn pop(&self) -> Option<Activity> {
        self.pending
            .lock()
            .expect("parent activity lock poisoned")
            .pop_front()
    }

    pub(super) fn remove(&self, activity: Activity) {
        let mut pending = self.pending.lock().expect("parent activity lock poisoned");
        if let Some(index) = pending.iter().position(|item| *item == activity) {
            pending.remove(index);
        }
    }
}
