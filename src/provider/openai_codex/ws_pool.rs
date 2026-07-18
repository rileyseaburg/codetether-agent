//! Exclusive recycling of a completed Responses WebSocket connection.

use super::OpenAiRealtimeConnection;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::Mutex;

#[path = "ws_pool/insert.rs"]
mod insert;

type Connections<S> = Arc<Mutex<HashMap<String, OpenAiRealtimeConnection<S>>>>;

pub(super) struct WsPool<S = tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>> {
    connections: Connections<S>,
}

impl<S> Clone for WsPool<S> {
    fn clone(&self) -> Self {
        Self {
            connections: Arc::clone(&self.connections),
        }
    }
}

impl<S> Default for WsPool<S> {
    fn default() -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl<S> WsPool<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    pub(super) async fn take(&self, session_id: &str) -> Option<OpenAiRealtimeConnection<S>> {
        self.connections.lock().await.remove(session_id)
    }

    pub(super) async fn put(&self, session_id: String, connection: OpenAiRealtimeConnection<S>) {
        insert::put(&self.connections, session_id, connection).await;
    }
}
