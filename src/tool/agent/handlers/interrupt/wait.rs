//! Notification-driven wait for an aborted child turn to release its session.

use super::super::super::{collaboration_runtime::execution_notify, execution_state};

pub(super) async fn until_idle(agent_id: &str) {
    let notify = execution_notify::notification(agent_id);
    while execution_state::is_running(agent_id) {
        let settled = notify.notified();
        tokio::pin!(settled);
        settled.as_mut().enable();
        if execution_state::is_running(agent_id) {
            settled.await;
        }
    }
}
