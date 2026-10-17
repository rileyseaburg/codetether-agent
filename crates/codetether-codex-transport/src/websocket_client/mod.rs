//! Session-scoped connection setup, reuse, and stream startup.
//!
//! [WebSocketTransport] borrows existing provider state. Payload preparation
//! and credentials remain the caller's responsibility.
use crate::websocket::{
    build_chatgpt_responses_ws_request, build_responses_ws_request_with_account_id,
};
use crate::{
    Chunks, OpenAiRealtimeConnection, TransportHealth, TurnStateStore, WsPool,
    X_CODEX_TURN_STATE_HEADER, websocket,
};
use anyhow::{Context, Result};
use codetether_codex_protocol::request::ResponsesWsBackend;
use serde_json::{Value, json};
use tokio_tungstenite::connect_async;
/// Borrowed session resources used to start a WebSocket response stream.
///
/// # Examples
/// ```
/// use codetether_codex_transport::{WebSocketTransport,TransportHealth,TurnStateStore,WsPool};
/// let health = TransportHealth::default();
/// let states = TurnStateStore::default();
/// let pool: WsPool = WsPool::default();
/// let transport = WebSocketTransport::new(&health, &states, &pool);
/// ```
pub struct WebSocketTransport<'a> {
    transport_health: &'a TransportHealth,
    turn_states: &'a TurnStateStore,
    ws_pool: &'a WsPool,
}
impl<'a> WebSocketTransport<'a> {
    /// Borrow existing health, routing, and pooled connection state.
    pub fn new(
        transport_health: &'a TransportHealth,
        turn_states: &'a TurnStateStore,
        ws_pool: &'a WsPool,
    ) -> Self {
        Self {
            transport_health,
            turn_states,
            ws_pool,
        }
    }
}
mod connect;
mod pooled;
mod stream;
