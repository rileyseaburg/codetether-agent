//! Exclusive recycling of a completed Responses WebSocket connection.

use super::OpenAiRealtimeConnection;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::Mutex;

pub(super) struct WsPool<S = tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>> {
    connection: Arc<Mutex<Option<OpenAiRealtimeConnection<S>>>>,
}

impl<S> Clone for WsPool<S> {
    fn clone(&self) -> Self {
        Self {
            connection: Arc::clone(&self.connection),
        }
    }
}

impl<S> Default for WsPool<S> {
    fn default() -> Self {
        Self {
            connection: Arc::new(Mutex::new(None)),
        }
    }
}

impl<S> WsPool<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    pub(super) async fn take(&self) -> Option<OpenAiRealtimeConnection<S>> {
        self.connection.lock().await.take()
    }

    pub(super) async fn put(&self, mut connection: OpenAiRealtimeConnection<S>) {
        let mut slot = self.connection.lock().await;
        if slot.is_none() {
            *slot = Some(connection);
            return;
        }
        drop(slot);
        let _ = connection.close().await;
    }
}
