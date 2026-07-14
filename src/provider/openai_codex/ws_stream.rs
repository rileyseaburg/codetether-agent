//! Transactional driver for one Codex WebSocket response.

use futures::stream::BoxStream;
use serde_json::Value;
use tokio::io::{AsyncRead, AsyncWrite};

use super::{
    OpenAiCodexProvider, OpenAiRealtimeConnection, ResponsesSseParser, StreamChunk, TransportHealth,
};

/// Buffer one WebSocket attempt until completion so interrupted output stays private.
pub(super) fn drive<S>(
    mut connection: OpenAiRealtimeConnection<S>,
    body: Value,
    health: TransportHealth,
) -> BoxStream<'static, StreamChunk>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    Box::pin(async_stream::stream! {
        if let Err(error) = connection.send_event(&body).await {
            interrupted(&health, Some(&error));
            let _ = connection.close().await;
            return;
        }
        let mut parser = ResponsesSseParser::default();
        let mut buffered = Vec::new();
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
                        break;
                    }
                }
                Ok(None) => { interrupted(&health, None); break; }
                Err(error) => { interrupted(&health, Some(&error)); break; }
            }
        }
        let _ = connection.close().await;
    })
}

fn interrupted(health: &TransportHealth, error: Option<&anyhow::Error>) {
    tracing::warn!(
        error = error.map(ToString::to_string),
        "Codex transport retrying privately"
    );
    health.mark_interrupted();
}
