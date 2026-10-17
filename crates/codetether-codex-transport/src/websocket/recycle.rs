//! Session-scoped recycling or closure after a WebSocket response.

use super::super::{OpenAiRealtimeConnection, TransportHealth, WsPool};
use tokio::io::{AsyncRead, AsyncWrite};

pub(super) async fn finish<S>(
    mut connection: OpenAiRealtimeConnection<S>,
    session_id: String,
    reusable: bool,
    health: &TransportHealth,
    pool: &WsPool<S>,
) where
    S: AsyncRead + AsyncWrite + Unpin,
{
    if reusable && !health.requires_http(&session_id) {
        pool.put(session_id, connection).await;
    } else {
        let _ = connection.close().await;
    }
}
