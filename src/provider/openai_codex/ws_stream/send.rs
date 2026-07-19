//! Idle-bounded initial WebSocket request send.

use serde_json::Value;
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncWrite};

use super::super::OpenAiRealtimeConnection;

pub(super) async fn request<S>(
    connection: &mut OpenAiRealtimeConnection<S>,
    body: &Value,
    session_id: &str,
) -> Result<(), String>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let sent = tokio::time::timeout(Duration::from_secs(300), connection.send_event(body)).await;
    let reason = match sent {
        Ok(Ok(())) => return Ok(()),
        Ok(Err(error)) => format!("failed to send websocket request: {error:#}"),
        Err(_) => "idle timeout sending websocket request".into(),
    };
    super::interrupted::record(session_id, &reason);
    let _ = connection.close().await;
    Err(format!("codex-retryable: {reason}"))
}
