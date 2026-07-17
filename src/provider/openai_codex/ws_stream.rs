//! Transactional driver for one Codex WebSocket response.

#[path = "ws_stream/interrupted.rs"]
mod interrupted;

use futures::stream::BoxStream;
use serde_json::Value;
use tokio::io::{AsyncRead, AsyncWrite};

use super::{
    OpenAiCodexProvider, OpenAiRealtimeConnection, ResponsesSseParser, StreamChunk,
    TransportHealth, WsPool,
};

/// Buffer one WebSocket attempt until completion so interrupted output stays private.
pub(super) fn drive<S>(
    mut connection: OpenAiRealtimeConnection<S>,
    body: Value,
    health: TransportHealth,
    pool: WsPool<S>,
) -> BoxStream<'static, StreamChunk>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    Box::pin(async_stream::stream! {
        if let Err(error) = connection.send_event(&body).await {
            interrupted::record(&health, Some(&error));
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
                        let error = buffered.pop().expect("error chunk exists");
                        yield error;
                        break;
                    }
                    if buffered.iter().any(|chunk| matches!(chunk, StreamChunk::Done { .. })) {
                        for chunk in buffered.drain(..) { yield chunk; }
                        reusable = true;
                        break;
                    }
                }
                Ok(None) => { interrupted::record(&health, None); break; }
                Err(error) => { interrupted::record(&health, Some(&error)); break; }
            }
        }
        if reusable { pool.put(connection).await; }
        else { let _ = connection.close().await; }
    })
}
