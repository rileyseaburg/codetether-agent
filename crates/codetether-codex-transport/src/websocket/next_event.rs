//! Idle-bounded WebSocket event receipt.

use serde_json::Value;
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncWrite};

use super::super::OpenAiRealtimeConnection;

pub(super) enum Next {
    Event(Value),
    Activity,
    Failed(String),
}

pub(super) async fn get<S>(connection: &mut OpenAiRealtimeConnection<S>, session_id: &str) -> Next
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let received =
        tokio::time::timeout(Duration::from_secs(300), super::receive::next(connection)).await;
    let next = match received {
        Ok(Ok(super::receive::Received::Event(event))) => Next::Event(event),
        Ok(Ok(super::receive::Received::Activity)) => Next::Activity,
        Ok(Ok(super::receive::Received::Closed)) => {
            Next::Failed("codex-retryable: stream closed before response.completed".into())
        }
        Ok(Ok(super::receive::Received::ClosedByServer)) => Next::Failed(
            "codex-retryable: websocket closed by server before response.completed".into(),
        ),
        Ok(Err(error)) => Next::Failed(format!("codex-retryable: {error:#}")),
        Err(_) => Next::Failed("codex-retryable: idle timeout waiting for websocket".into()),
    };
    if let Next::Failed(reason) = &next {
        super::interrupted::record(session_id, reason);
    }
    next
}
