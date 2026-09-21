//! Bounded live token delivery with transport heartbeats during native prefill.
use super::BonsaiProvider;
use crate::provider::{CompletionRequest, StreamChunk};
use anyhow::Result;
use futures::{StreamExt, stream::BoxStream};
pub(super) fn start(
    provider: &BonsaiProvider,
    request: CompletionRequest,
) -> Result<BoxStream<'static, StreamChunk>> {
    #[cfg(not(feature = "candle-cuda"))]
    {
        let _ = (provider, request);
        anyhow::bail!("Bonsai requires candle-cuda");
    }
    #[cfg(feature = "candle-cuda")]
    {
        let provider = provider.clone();
        let (sender, receiver) = tokio::sync::mpsc::channel(32);
        tokio::task::spawn_blocking(move || {
            if let Err(error) = super::worker::run(&provider, request, &sender) {
                let _ = sender.blocking_send(StreamChunk::Error(format!("{error:#}")));
            }
        });
        Ok(futures::stream::unfold(receiver, |mut receiver| async move {
            let item = tokio::select! {
                item = receiver.recv() => item,
                _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => Some(StreamChunk::KeepAlive),
            };
            item.map(|item| (item, receiver))
        }).boxed())
    }
}
