//! UDP LAN broadcast discovery for A2A peers.
//!
//! Some Windows, VM, and router setups block DNS-SD browsing. This module
//! sends a tiny JSON beacon on a CodeTether-specific UDP port and feeds
//! discovered URLs into the same A2A peer intake path used by mDNS.

mod beacon;
mod endpoint;
mod recv;
mod send;
mod socket;

use crate::a2a::mdns::DiscoveredPeer;
use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::{sync::mpsc, task::JoinHandle};

/// Starts UDP broadcast announce and listen tasks for LAN A2A discovery.
pub async fn announce_and_listen(
    name: String,
    public_url: String,
    collaboration_token: String,
    peer_tx: mpsc::Sender<DiscoveredPeer>,
) -> Result<Vec<JoinHandle<()>>> {
    let socket = socket::bind().context("failed to bind A2A LAN discovery UDP socket")?;
    let socket = Arc::new(socket);
    Ok(vec![
        tokio::spawn(send::run(
            socket.clone(),
            name.clone(),
            public_url.clone(),
            collaboration_token,
        )),
        tokio::spawn(recv::run(socket, name, public_url, peer_tx)),
    ])
}
