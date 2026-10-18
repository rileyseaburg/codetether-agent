//! Bind the available IPv4 and IPv6 loopback callback sockets.
use crate::{CODEX_CALLBACK_ADDR_V4, CODEX_CALLBACK_ADDR_V6};
use tokio::net::TcpListener;

pub(super) async fn listeners() -> Vec<TcpListener> {
    let mut listeners = Vec::new();
    for address in [CODEX_CALLBACK_ADDR_V4, CODEX_CALLBACK_ADDR_V6] {
        match TcpListener::bind(address).await {
            Ok(listener) => listeners.push(listener),
            Err(error) => {
                tracing::debug!(
                    address,
                    error = %error,
                    "Failed to bind one OAuth callback listener address"
                );
            }
        }
    }

    listeners
}
