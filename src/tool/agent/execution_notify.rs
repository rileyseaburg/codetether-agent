//! Completion notifications for child-agent state transitions.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::Notify;

lazy_static::lazy_static! {
    static ref NOTIFIERS: Mutex<HashMap<String, Arc<Notify>>> = Mutex::new(HashMap::new());
}

pub(crate) fn notification(name: &str) -> Arc<Notify> {
    NOTIFIERS
        .lock()
        .expect("agent notifier lock poisoned")
        .entry(name.to_string())
        .or_insert_with(|| Arc::new(Notify::new()))
        .clone()
}

pub(crate) fn signal(name: &str) {
    let notify = notification(name);
    notify.notify_waiters();
    notify.notify_one();
}
