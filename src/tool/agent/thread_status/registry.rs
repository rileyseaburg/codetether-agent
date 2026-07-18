//! Watch channels keyed by canonical child session ID.

use super::ThreadStatus;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use tokio::sync::watch;

static STATUS: LazyLock<Mutex<HashMap<String, watch::Sender<ThreadStatus>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub(crate) fn initialize(agent_id: &str) {
    let mut status = STATUS.lock().expect("thread status lock poisoned");
    status.entry(agent_id.into()).or_insert_with(|| {
        let (tx, _) = watch::channel(ThreadStatus::PendingInit);
        tx
    });
}

pub(crate) fn set(agent_id: &str, next: ThreadStatus) {
    let mut status = STATUS.lock().expect("thread status lock poisoned");
    let sender = status.entry(agent_id.into()).or_insert_with(|| {
        let (tx, _) = watch::channel(next.clone());
        tx
    });
    sender.send_replace(next);
}

pub(crate) fn get(agent_id: &str) -> ThreadStatus {
    STATUS
        .lock()
        .ok()
        .and_then(|status| status.get(agent_id).map(|tx| tx.borrow().clone()))
        .unwrap_or(ThreadStatus::NotFound)
}

pub(crate) fn subscribe(agent_id: &str) -> Option<watch::Receiver<ThreadStatus>> {
    STATUS
        .lock()
        .ok()?
        .get(agent_id)
        .map(watch::Sender::subscribe)
}

pub(crate) fn remove(agent_id: &str) {
    if let Ok(mut status) = STATUS.lock() {
        if let Some(sender) = status.remove(agent_id) {
            sender.send_replace(ThreadStatus::NotFound);
        }
    }
}
