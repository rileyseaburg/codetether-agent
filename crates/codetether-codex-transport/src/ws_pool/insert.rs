//! Bounded insertion and eviction for idle session connections.

use super::{Connections, OpenAiRealtimeConnection};
use tokio::io::{AsyncRead, AsyncWrite};

const MAX_IDLE_SESSIONS: usize = 8;

pub(super) async fn put<S>(
    connections: &Connections<S>,
    session_id: String,
    mut connection: OpenAiRealtimeConnection<S>,
) where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let mut connections = connections.lock().await;
    if connections.contains_key(&session_id) {
        drop(connections);
        let _ = connection.close().await;
        return;
    }
    let evicted = if connections.len() >= MAX_IDLE_SESSIONS {
        connections
            .keys()
            .next()
            .cloned()
            .and_then(|key| connections.remove(&key))
    } else {
        None
    };
    connections.insert(session_id, connection);
    drop(connections);
    if let Some(mut evicted) = evicted {
        let _ = evicted.close().await;
    }
}
