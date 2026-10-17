//! Connection new.
use super::*;

impl<S> OpenAiRealtimeConnection<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    /// Wrap an already-connected WebSocket for JSON event I/O.
    pub fn new(stream: WebSocketStream<S>) -> Self {
        Self { stream }
    }
}
