//! Consume native text events, flushing output and requiring explicit completion.
use anyhow::{Result, ensure};
use codetether_provider_core::StreamChunk;
use futures::{StreamExt, stream::BoxStream};
use std::io::Write;
pub(super) async fn consume(stream: &mut BoxStream<'static, StreamChunk>) -> Result<()> {
    let mut done = false;
    while let Some(chunk) = stream.next().await {
        match chunk {
            StreamChunk::Text(text) => {
                print!("{text}");
                std::io::stdout().flush()?;
            }
            StreamChunk::Done { .. } => {
                done = true;
                break;
            }
            StreamChunk::Error(error) => anyhow::bail!(error),
            StreamChunk::KeepAlive => {}
            other => anyhow::bail!("Unexpected Bonsai event: {other:?}"),
        }
    }
    ensure!(done, "Bonsai stream ended without completion");

    Ok(())
}
