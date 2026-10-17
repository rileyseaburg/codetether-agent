//! Pending registry, lock ownership, and cancellation cleanup.
use crate::{Answers, Draft, Error};
use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex, MutexGuard},
};
use tokio::sync::oneshot;
static PENDING: LazyLock<Mutex<HashMap<String, Pending>>> = LazyLock::new(Mutex::default);
pub(super) struct Pending {
    pub(super) draft: Draft,
    pub(super) reply: Option<oneshot::Sender<Result<Answers, Error>>>,
}
pub(super) struct Cleanup(pub(super) String);
pub(super) fn lock() -> MutexGuard<'static, HashMap<String, Pending>> {
    PENDING
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

impl Drop for Cleanup {
    fn drop(&mut self) {
        lock().remove(&self.0);
    }
}
