//! Wait until a starting child accepts steering or becomes idle.

use crate::Host;
use codetether_collaboration_state::{
    execution as execution_state, execution_notify, thread_status,
};

pub(super) async fn steer_or_idle<H: Host>(agent_id: &str, message: &str) -> bool {
    let notify = execution_notify::notification(agent_id);
    let mut status = thread_status::subscribe(agent_id);
    loop {
        if H::push(agent_id, message) {
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
