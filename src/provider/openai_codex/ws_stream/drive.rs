//! Transactional driver for one Codex WebSocket response.

use futures::stream::BoxStream;
use serde_json::Value;
use tokio::io::{AsyncRead, AsyncWrite};

use super::super::{
    OpenAiRealtimeConnection, ResponsesSseParser, StreamChunk, TransportHealth, TurnStateStore,
    WsPool,
};

pub(in super::super) fn run<S>(
    mut connection: OpenAiRealtimeConnection<S>,
    body: Value,
    session_id: String,
    health: TransportHealth,
    turn_states: TurnStateStore,
    pool: WsPool<S>,
) -> BoxStream<'static, StreamChunk>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    Box::pin(async_stream::stream! {
        if let Err(reason) = super::send::request(&mut connection, &body, &session_id).await {
            yield StreamChunk::Error(reason);
            return;
        }
        let mut parser = ResponsesSseParser::default();
        let mut reusable = false;
        loop {
            let event = match super::next_event::get(&mut connection, &session_id).await {
                super::next_event::Next::Event(event) => event,
                super::next_event::Next::Activity => {
                    yield StreamChunk::KeepAlive;
                    continue;
                }
                super::next_event::Next::Failed(reason) => {
                    yield StreamChunk::Error(reason);
                    break;
                }
            };
            super::turn_state::capture(&turn_states, &session_id, &event);
            let parsed = super::parse::event(&mut parser, &event);
            for chunk in parsed.chunks { yield chunk; }
            if parsed.terminal {
                reusable = parsed.reusable;
                break;
            }
        }
        super::recycle::finish(connection, session_id, reusable, &health, &pool).await;
    })
}
