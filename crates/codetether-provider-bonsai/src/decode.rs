//! Autoregressive token sampling; incremental decoder retains partial UTF-8 safely.
use super::{GenerationTiming, request::GenerationRequest, runtime::Runtime};
use anyhow::{Result, ensure};
use candle_core::Tensor;
use std::time::Instant;
pub(super) fn run(
    runtime: &mut Runtime,
    request: &GenerationRequest,
    mut logits: Tensor,
    input_len: usize,
    started: Instant,
    cancelled: &dyn Fn() -> bool,
    emit: &mut dyn FnMut(&str) -> Result<bool>,
) -> Result<GenerationTiming> {
    let mut sampler = super::sampler::new(runtime, request);
    let loaded = &mut runtime.loaded;
    let mut decoder = loaded.tokenizer.decode_stream(false);
    let mut timing = GenerationTiming::default();
    let mut first = None;
    for index in 0..request.max_tokens {
        ensure!(!cancelled(), "Bonsai generation cancelled");
        let token = sampler.sample(&logits)?;
        if loaded.eos.contains(&token) {
            break;
        }
        let now = Instant::now();
        let initial = *first.get_or_insert(now);
        timing.ttft_ms = initial.duration_since(started).as_secs_f64() * 1000.0;
        timing.decode_ms = now.duration_since(initial).as_secs_f64() * 1000.0;
        timing.generated_tokens += 1;
        let delta = decoder
            .step(token)
            .map_err(|e| anyhow::anyhow!("Decode Bonsai token: {e}"))?;
        if let Some(text) = delta
            && !emit(&text)?
        {
            break;
        }
        if index + 1 == request.max_tokens {
            break;
        }
        logits = loaded
            .model
            .forward_tokens(&[token], input_len + index, cancelled)?
            .squeeze(0)?;
    }
    loaded.device.synchronize()?;
    timing.total_ms = started.elapsed().as_secs_f64() * 1000.0;
    Ok(timing)
}
