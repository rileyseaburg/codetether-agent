//! Poll upstream and bounded output under one deadline; receiver drop aborts both.
use codetether_provider_core::StreamChunk;
use futures::{StreamExt, stream::BoxStream};
use tokio::{
    sync::mpsc::Sender,
    time::{Instant, timeout_at},
};

pub(super) async fn forward(
    mut stream: BoxStream<'static, StreamChunk>,
    sender: &Sender<StreamChunk>,
    deadline: Instant,
) -> Option<StreamChunk> {
    loop {
        let next = tokio::select! {
            biased;
            _ = sender.closed() => return None,
            next = timeout_at(deadline, stream.next()) => next,
        };
        match next {
            Ok(Some(chunk @ (StreamChunk::Done { .. } | StreamChunk::Error(_)))) => {
                return Some(chunk);
            }
            Ok(Some(chunk)) => match timeout_at(deadline, sender.send(chunk)).await {
                Ok(Ok(())) => {}
                Ok(Err(_)) => return None,
                Err(_) => return Some(expired()),
            },
            Ok(None) => {
                return Some(StreamChunk::Error(
                    "LM Studio stream ended without a completion marker".into(),
                ));
            }
            Err(_) => return Some(expired()),
        }
    }
}
fn expired() -> StreamChunk {
    StreamChunk::Error("LM Studio generation deadline exceeded".into())
}
