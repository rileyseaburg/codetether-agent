//! Bounded best-effort pause notification before a replacement worker starts.
use crate::relay::Device;
use std::time::Duration;
use tokio_util::sync::CancellationToken;

pub(super) async fn notify(device: &Device) {
    if !device.expired() {
        let cancel = CancellationToken::new();
        let _ = tokio::time::timeout(Duration::from_secs(3), device.pause_remote(&cancel)).await;
    }
}
