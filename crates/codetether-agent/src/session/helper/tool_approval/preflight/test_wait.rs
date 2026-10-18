//! Wait for a fixture's background check without modifying the shared test clock.

use std::path::Path;
use tokio::time::{Duration, timeout};

pub(super) async fn ready(root: &Path, limit: Duration) {
    timeout(limit, async {
        while super::cooldown::reason(root, "typescript").is_some() {
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("background diagnostics did not finish");
}
