//! Stream-event assembly after a validated response.
use crate::*;
use futures::StreamExt as _;
impl GeminiWebProvider {
    pub(crate) async fn stream_response(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        let prompt = prompt::render(&request.messages, &request.tools)
            .context("Failed to render Gemini Web streaming prompt")?;
        let (cleaned, calls) = self
            .ask_validated(&prompt, &request.model, &request.tools, &request.messages)
            .await
            .context("Gemini Web streaming completion failed")?;
        let usage = usage::estimate(&prompt, &cleaned, &calls);
        let chunks = stream_output::from_parts(cleaned, calls, usage);
        Ok(futures::stream::iter(chunks).boxed())
    }
}
