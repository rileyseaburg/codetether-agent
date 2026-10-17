//! Typed Responses connection with JSON I/O and control-frame handling.
//!
//! [OpenAiRealtimeConnection] wraps an already-connected socket. Call
//! send_event/recv_event for direct JSON I/O or hand it to websocket::drive.
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
/// Typed WebSocket connection for OpenAI Responses events.
///
/// The connection serializes and parses each event as JSON while handling
/// WebSocket control frames internally.
///
/// # Examples
///
/// ```no_run
/// use codetether_codex_transport::OpenAiRealtimeConnection;
/// fn wrap<S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin>(
///     socket: tokio_tungstenite::WebSocketStream<S>,
/// ) -> OpenAiRealtimeConnection<S> {
///     OpenAiRealtimeConnection::new(socket)
/// }
/// ```
pub struct OpenAiRealtimeConnection<S = MaybeTlsStream<TcpStream>> {
    pub(crate) stream: WebSocketStream<S>,
}

impl<S> std::fmt::Debug for OpenAiRealtimeConnection<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenAiRealtimeConnection").finish()
    }
}
mod close;
mod construct;
mod receive;
mod send;
