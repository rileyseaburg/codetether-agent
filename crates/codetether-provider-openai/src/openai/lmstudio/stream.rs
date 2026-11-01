//! Bounded stream forwarding: stalled consumers cannot hold a generation forever.
use super::gate::Lease;
use codetether_provider_core::StreamChunk;
use futures::{
    StreamExt,
    stream::{BoxStream, unfold},
};
use tokio::sync::{mpsc, oneshot};
mod pump;

pub(super) fn leased(
    stream: BoxStream<'static, StreamChunk>,
    lease: Lease,
) -> BoxStream<'static, StreamChunk> {
    let (sender, receiver) = mpsc::channel(8);
    let (finish, terminal) = oneshot::channel();
    tokio::spawn(async move {
        let end = pump::forward(stream, &sender, lease.deadline).await;
        drop(lease);
        drop(sender);
        let _ = finish.send(end);
    });
    unfold(Some((receiver, terminal)), |state| async move {
        let (mut receiver, terminal) = state?;
        if let Some(chunk) = receiver.recv().await {
            return Some((chunk, Some((receiver, terminal))));
        }
        terminal
            .await
            .unwrap_or_else(|_| Some(StreamChunk::Error("LM Studio stream worker stopped".into())))
            .map(|chunk| (chunk, None))
    })
    .boxed()
}
#[cfg(test)]
mod tests;
