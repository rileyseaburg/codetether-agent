//! Bounded, retrying transport for no-LLM introduction messages.

use crate::a2a::{client::A2AClient, types::MessageSendParams};
use anyhow::{Result, anyhow};
use std::time::Duration;

const ATTEMPTS: u32 = 3;
const DEADLINE: Duration = Duration::from_secs(4);

pub(super) async fn send(
    endpoint: &str,
    token: Option<&str>,
    payload: MessageSendParams,
) -> Result<()> {
    let mut last_error = anyhow!("introduction delivery did not run");
    for attempt in 0..ATTEMPTS {
        let mut client = A2AClient::new(endpoint);
        if let Some(token) = token {
            client = client.with_token(token);
        }
        match tokio::time::timeout(DEADLINE, client.send_message(payload.clone())).await {
            Ok(Ok(_)) => return Ok(()),
            Ok(Err(error)) => last_error = error,
            Err(_) => last_error = anyhow!("introduction delivery timed out after {DEADLINE:?}"),
        }
        tracing::debug!(peer = %endpoint, attempt = attempt + 1, error = %last_error,
            "A2A introduction delivery attempt failed");
        if attempt + 1 < ATTEMPTS {
            tokio::time::sleep(Duration::from_millis(150 * 2_u64.pow(attempt))).await;
        }
    }
    Err(last_error)
}
