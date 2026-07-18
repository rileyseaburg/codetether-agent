//! Transactional driver for one Codex WebSocket response.

use futures::stream::BoxStream;
use serde_json::Value;
use tokio::io::{AsyncRead, AsyncWrite};

use super::super::{
    OpenAiCodexProvider, OpenAiRealtimeConnection, ResponsesSseParser, StreamChunk,
    TransportHealth, WsPool,
};

pub(in super::super) fn run<S>(
    mut connection: OpenAiRealtimeConnection<S>,
    body: Value,
    session_id: String,
    health: TransportHealth,
    pool: WsPool<S>,
) -> BoxStream<'static, StreamChunk>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    Box::pin(async_stream::stream! {
        if let Err(error) = connection.send_event(&body).await {
            super::interrupted::record(&health, &session_id, Some(&error));
            let _ = connection.close().await;
            return;
        }
        let mut parser = ResponsesSseParser::default();
        let mut buffered = Vec::new();
        let mut reusable = false;
        loop {
            match connection.recv_event().await {
                Ok(Some(event)) => {
                    OpenAiCodexProvider::parse_responses_event(&mut parser, &event, &mut buffered);
                    if buffered.iter().any(|chunk| matches!(chunk, StreamChunk::Error(_))) {
                        yield buffered.pop().expect("error chunk exists");
                        break;
                    }
                    if buffered.iter().any(|chunk| matches!(chunk, StreamChunk::Done { .. })) {
                        for chunk in buffered.drain(..) { yield chunk; }
                        reusable = true;
                        break;
                    }
                }
                Ok(None) => { super::interrupted::record(&health, &session_id, None); break; }
                Err(error) => { super::interrupted::record(&health, &session_id, Some(&error)); break; }
            }
        }
        super::recycle::finish(connection, session_id, reusable, &health, &pool).await;
    })
}
