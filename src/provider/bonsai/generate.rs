//! Direct CUDA generation, incremental UTF-8 decoding and measured phase boundaries.
use super::{GenerationTiming, request::GenerationRequest, runtime::Runtime};
use anyhow::{Result, ensure};
use std::time::Instant;
pub(super) fn run(
    runtime: &mut Runtime,
    request: &GenerationRequest,
    cancelled: &dyn Fn() -> bool,
    emit: &mut dyn FnMut(&str) -> Result<bool>,
) -> Result<(usize, usize, GenerationTiming)> {
    let started = Instant::now();
    let loaded = &mut runtime.loaded;
    let encoded = loaded
        .tokenizer
        .encode(request.prompt.as_str(), false)
        .map_err(|error| anyhow::anyhow!("Tokenize Bonsai prompt: {error}"))?;
    let tokens = encoded.get_ids();
    ensure!(
        !tokens.is_empty() && tokens.len() + request.max_tokens <= loaded.model.context(),
        "Bonsai context exceeded; prompt is not silently truncated"
    );
    loaded.model.clear();
    let prefill = Instant::now();
    let logits = loaded
        .model
        .forward_tokens(tokens, 0, cancelled)?
        .squeeze(0)?;
    loaded.device.synchronize()?;
    let prefill_ms = prefill.elapsed().as_secs_f64() * 1000.0;
    let mut timing = super::decode::run(
        runtime,
        request,
        logits,
        tokens.len(),
        started,
        cancelled,
        emit,
    )?;
    timing.prefill_ms = prefill_ms;
    let generated = timing.generated_tokens;
    Ok((tokens.len(), generated, timing))
}
