//! Race-free wait for queued or future parent activity.

use super::{Activity, channel::Channel};
use std::sync::Arc;
use tokio::time::Instant;

pub(super) async fn until(channel: Arc<Channel>, deadline: Instant) -> Option<Activity> {
    loop {
        if let Some(activity) = channel.pop() {
            return Some(activity);
        }
        let notified = channel.notify.notified();
        tokio::pin!(notified);
        notified.as_mut().enable();
        if let Some(activity) = channel.pop() {
            return Some(activity);
        }
        if tokio::time::timeout_at(deadline, &mut notified)
            .await
            .is_err()
        {
            return None;
        }
    }
}
