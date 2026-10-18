//! OAuth callback wait handling.
use crate::{read::read_http_request, request::parse_oauth_callback_request, response};
use anyhow::{Context, Result};
use tokio::net::TcpListener;
use tokio::time::{Duration, Instant};

pub(super) async fn wait_for_oauth_callback(
    listener: TcpListener,
    timeout: Duration,
) -> Result<(String, String)> {
    let deadline = Instant::now() + timeout;

    loop {
        let now = Instant::now();
        if now >= deadline {
            anyhow::bail!("Timed out waiting for OAuth callback");
        }
        let remaining = deadline - now;

        let (mut stream, peer_addr) = tokio::time::timeout(remaining, listener.accept())
            .await
            .context("Timed out waiting for callback connection")?
            .context("Failed to accept callback connection")?;

        let request = read_http_request(&mut stream).await?;
        match parse_oauth_callback_request(&request) {
            Ok((code, state)) => {
                response::accepted(&mut stream).await?;
                return Ok((code, state));
            }
            Err(error) => {
                tracing::warn!(
                    peer = %peer_addr,
                    error = %error,
                    "Ignoring non-callback HTTP request while waiting for OAuth callback"
                );
                response::rejected(&mut stream).await?;
            }
        }
    }
}
