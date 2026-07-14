//! Direct cancellation signal shared by the TUI and session runtime.

use std::sync::Arc;

use parking_lot::Mutex;
use tokio::sync::Notify;

#[derive(Clone, Default)]
pub(super) struct ActiveCancel(Arc<Mutex<Option<Arc<Notify>>>>);

impl ActiveCancel {
    pub(super) fn set(&self, notify: Arc<Notify>) -> bool {
        let mut active = self.0.lock();
        if active.is_some() {
            return false;
        }
        *active = Some(notify);
        true
    }

    pub(super) fn clear(&self) {
        *self.0.lock() = None;
    }

    pub(super) fn notify(&self) -> bool {
        let active = self.0.lock().clone();
        if let Some(notify) = active {
            notify.notify_one();
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use futures::FutureExt;

    use super::*;

    #[test]
    fn notifies_only_while_a_turn_is_active() {
        let active = ActiveCancel::default();
        let notify = Arc::new(Notify::new());
        assert!(!active.notify());
        assert!(active.set(Arc::clone(&notify)));
        assert!(active.notify());
        assert!(notify.notified().now_or_never().is_some());
        active.clear();
        assert!(!active.notify());
    }
}
