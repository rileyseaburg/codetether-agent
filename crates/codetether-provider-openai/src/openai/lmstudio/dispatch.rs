//! LM Studio admission precedes fresh context validation and upstream generation.
use super::{config::Config, registry, stream};
use crate::openai::*;
use anyhow::Context;
use tokio::time::timeout_at;

impl OpenAIProvider {
    pub(in crate::openai) async fn local_complete(
        &self,
        request: CompletionRequest,
    ) -> Result<CompletionResponse> {
        let gate = registry::gate(&self.api_base, Config::load()?)?;
        let lease = gate.acquire().await?;
        timeout_at(lease.deadline, async {
            let request = self.guard_lmstudio_request(request).await?;
            self.complete_request(request).await
        })
        .await
        .context("LM Studio generation deadline exceeded")?
    }
    pub(in crate::openai) async fn local_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        let gate = registry::gate(&self.api_base, Config::load()?)?;
        let lease = gate.acquire().await?;
        let stream = timeout_at(lease.deadline, async {
            let request = self.guard_lmstudio_request(request).await?;
            self.stream_request(request).await
        })
        .await
        .context("LM Studio generation deadline exceeded")??;
        Ok(stream::leased(stream, lease))
    }
}
