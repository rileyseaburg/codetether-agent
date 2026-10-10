//! Shared one-way cancellation for a single background worker generation.
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tokio_util::sync::CancellationToken;

#[derive(Clone)]
pub(super) struct Stop {
    pub(super) flag: Arc<AtomicBool>,
    pub(super) token: CancellationToken,
    signal: Arc<AtomicBool>,
}
impl Stop {
    pub(super) fn new(signal: Arc<AtomicBool>) -> Self {
        // The previous worker, including its capture task, has already finished.
        signal.store(false, Ordering::Release);
        Self {
            flag: Arc::new(AtomicBool::new(false)),
            signal,
            token: CancellationToken::new(),
        }
    }
    pub(super) fn cancel(&self) {
        self.flag.store(true, Ordering::Release);
        self.signal.store(true, Ordering::Release);
        self.token.cancel();
    }
    pub(super) fn check(&self) -> Result<(), crate::relay::Error> {
        if self.signal.load(Ordering::Acquire) || self.token.is_cancelled() {
            self.cancel();
            return Err(crate::relay::Error::Cancelled);
        }
        Ok(())
    }
    pub(super) async fn watch(&self) {
        while !self.signal.load(Ordering::Acquire) && !self.token.is_cancelled() {
            tokio::time::sleep(std::time::Duration::from_millis(16)).await;
        }
        self.cancel();
    }
}
