//! Device authorization timing responsibilities.
use crate::OAUTH_POLLING_SAFETY_MARGIN_MS;
use tokio::time::{Duration, sleep};

pub(super) async fn sleep_with_margin(interval_secs: u64) {
    sleep(Duration::from_millis(
        interval_secs.saturating_mul(1000) + OAUTH_POLLING_SAFETY_MARGIN_MS,
    ))
    .await;
}
