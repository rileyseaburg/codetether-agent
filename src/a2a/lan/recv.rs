//! UDP beacon receiver for LAN A2A discovery.

use super::beacon::Beacon;
use crate::a2a::mdns::DiscoveredPeer;
use std::sync::Arc;
use tokio::{net::UdpSocket, sync::mpsc};

pub async fn run(
    socket: Arc<UdpSocket>,
    self_name: String,
    self_url: String,
    peer_tx: mpsc::Sender<DiscoveredPeer>,
) {
    let mut buf = vec![0_u8; 2048];
    loop {
        let Ok((len, source)) = socket.recv_from(&mut buf).await else {
            continue;
        };
        let Ok(beacon) = serde_json::from_slice::<Beacon>(&buf[..len]) else {
            continue;
        };
        if !is_peer(&beacon, &self_name, &self_url) {
            continue;
        }
        tracing::debug!(
            peer_name = %beacon.name,
            source = %source,
            "A2A LAN beacon discovered"
        );
        if peer_tx.send(to_peer(beacon, source.ip())).await.is_err() {
            return;
        }
    }
}

fn is_peer(beacon: &Beacon, self_name: &str, self_url: &str) -> bool {
    beacon.is_valid() && beacon.name != self_name && beacon.url != self_url
}

fn to_peer(beacon: Beacon, source: std::net::IpAddr) -> DiscoveredPeer {
    DiscoveredPeer {
        urls: vec![super::endpoint::from_source(&beacon.url, source)],
        instance_name: beacon.name,
        token: Some(beacon.token),
    }
}
