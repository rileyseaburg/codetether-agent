//! Worker task release client.

pub(super) use super::release_payload::Payload;
use anyhow::Result;
use reqwest::Client;
pub(super) async fn send(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    payload: Payload<'_>,
) -> Result<()> {
    codetether_a2a_http::worker::send_release(request(client, server, token, worker_id, payload)?)
        .await
}

use codetether_a2a_http::worker::build_release_request as request;
