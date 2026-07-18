//! Wait until a starting child accepts steering or becomes idle.

use super::super::{collaboration_runtime, execution_state};

pub(super) async fn steer_or_idle(agent_id: &str, message: &str) -> bool {
    let notify = collaboration_runtime::execution_notify::notification(agent_id);
    let mut status = collaboration_runtime::thread_status::subscribe(agent_id);
    loop {
        if super::active::push(agent_id, message) {
            return true;
        }
        if !execution_state::is_running(agent_id) {
            return false;
        }
        let settled = notify.notified();
        tokio::pin!(settled);
        settled.as_mut().enable();
        if let Some(receiver) = status.as_mut() {
            tokio::select! {
                _ = receiver.changed() => {}
                _ = &mut settled => {}
            }
        } else {
            settled.await;
        }
    }
}
