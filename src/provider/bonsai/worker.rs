//! One serialized native generation request on a blocking worker.
use super::BonsaiProvider;
use crate::provider::{CompletionRequest, StreamChunk};
use anyhow::Result;
pub(super) fn run(
    provider: &BonsaiProvider,
    request: CompletionRequest,
    sender: &tokio::sync::mpsc::Sender<StreamChunk>,
) -> Result<()> {
    let request = super::request::GenerationRequest::parse(&request)?;
    let mut guard = provider
        .runtime
        .try_lock()
        .map_err(|_| anyhow::anyhow!("Bonsai is busy"))?;
    let load_ms = super::runtime::ensure_loaded(&mut guard, &provider.config)?;
    let mut emit = |text: &str| -> Result<()> {
        if !text.is_empty() {
            sender.blocking_send(StreamChunk::Text(text.into()))?;
        }
        Ok(())
    };
    let mut filter = super::stop::Filter::new(request.stop.clone());
    let (input, output, mut timing) = super::generate::run(
        guard.as_mut().unwrap(),
        &request,
        &|| sender.is_closed(),
        &mut |text| filter.push(text, &mut emit),
    )?;
    filter.finish(&mut emit)?;
    timing.load_ms = load_ms;
    tracing::info!(load_ms, prefill_ms=timing.prefill_ms, ttft_ms=timing.ttft_ms,
        decode_ms=timing.decode_ms, decode_tps=?timing.decode_tps(), "Native Bonsai generation");
    *provider
        .timing
        .lock()
        .map_err(|_| anyhow::anyhow!("Bonsai timing lock poisoned"))? = Some(timing);
    sender.blocking_send(StreamChunk::Done {
        usage: Some(crate::provider::Usage {
            prompt_tokens: input,
            completion_tokens: output,
            total_tokens: input + output,
            ..Default::default()
        }),
    })?;
    Ok(())
}
