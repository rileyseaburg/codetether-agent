//! Exclusive recycling of a completed Responses WebSocket connection.

use super::OpenAiRealtimeConnection;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::Mutex;

#[path = "ws_pool/insert.rs"]
mod insert;

type Connections<S> = Arc<Mutex<HashMap<String, OpenAiRealtimeConnection<S>>>>;

/// Session-keyed idle connections with exclusive take/put ownership.
///
/// # Examples
/// ```
/// let pool: codetether_codex_transport::WsPool = Default::default();
/// let shared = pool.clone();
/// ```
pub struct WsPool<S = tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>> {
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
    /// Remove and return the idle connection for a session.
    pub async fn take(&self, session_id: &str) -> Option<OpenAiRealtimeConnection<S>> {
        self.connections.lock().await.remove(session_id)
    }

    /// Store a connection, closing duplicates and one evicted idle entry.
    pub async fn put(&self, session_id: String, connection: OpenAiRealtimeConnection<S>) {
        insert::put(&self.connections, session_id, connection).await;
    }
}
