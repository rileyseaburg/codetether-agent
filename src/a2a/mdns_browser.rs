//! Background mDNS browse-loop lifecycle.

use super::{DiscoveredPeer, SERVICE_TYPE, event};
use anyhow::{Context, Result};
use mdns_sd::ServiceDaemon;
use std::sync::Arc;
use tokio::sync::mpsc;

pub(super) fn start(
    daemon: &Arc<ServiceDaemon>,
    self_fullname: String,
    self_port: u16,
    peer_tx: mpsc::Sender<DiscoveredPeer>,
) -> Result<()> {
    let receiver = daemon
        .browse(SERVICE_TYPE)
        .context("Failed to start mDNS browse")?;
    tokio::task::spawn_blocking(move || {
        while let Ok(service_event) = receiver.recv() {
            if !event::keep_browsing(service_event, &self_fullname, self_port, &peer_tx) {
                break;
            }
        }
    });
    Ok(())
}
