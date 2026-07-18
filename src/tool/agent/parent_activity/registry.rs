//! Parent-session channel registry.

use super::{Activity, channel::Channel};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};

static CHANNELS: LazyLock<Mutex<HashMap<String, Arc<Channel>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub(super) fn channel(owner: &str) -> Arc<Channel> {
    CHANNELS
        .lock()
        .expect("parent activity registry poisoned")
        .entry(owner.to_string())
        .or_default()
        .clone()
}

pub(super) fn publish(owner: &str, activity: Activity) {
    channel(owner).push(activity);
}

#[cfg(test)]
pub(super) fn clear(owner: &str) {
    CHANNELS
        .lock()
        .expect("parent activity registry poisoned")
        .remove(owner);
}
