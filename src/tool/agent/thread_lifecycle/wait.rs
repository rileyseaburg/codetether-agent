//! Notification-driven wait for a closing child turn to settle.

use super::super::{collaboration_runtime::execution_notify, execution_state};

pub(super) async fn until_idle(name: &str) {
    let notify = execution_notify::notification(name);
    while execution_state::is_running(name) {
        let notified = notify.notified();
        tokio::pin!(notified);
        notified.as_mut().enable();
        if execution_state::is_running(name) {
            notified.await;
        }
    }
}
