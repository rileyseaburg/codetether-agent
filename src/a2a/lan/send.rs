//! UDP beacon sender for LAN A2A discovery.

use super::beacon::{Beacon, PORT};
use std::sync::Arc;
use tokio::{net::UdpSocket, time::Duration};

pub async fn run(socket: Arc<UdpSocket>, name: String, url: String) {
    let payload = match serde_json::to_vec(&Beacon::new(name, url)) {
        Ok(payload) => payload,
        Err(error) => {
            tracing::warn!(%error, "failed to encode A2A LAN beacon");
            return;
        }
    };
    let addr = ("255.255.255.255", PORT);
    let mut tick = tokio::time::interval(Duration::from_secs(3));
    loop {
        tick.tick().await;
        let _ = socket.send_to(&payload, addr).await;
    }
}
