//! Bind the packed native Bonsai decoder to the existing Candle generation harness.
use super::super::{
    ThinkerConfig, candle_device, candle_model::CandleModel, candle_resolve,
    candle_runtime::CandleThinker,
};
use anyhow::Result;
pub(super) fn load(config: &ThinkerConfig) -> Result<CandleThinker> {
    let (path, tokenizer_path) = candle_resolve::paths(config)?;
    let (device, label) = candle_device::select_candle_device(config)?;
    let loaded = crate::provider::bonsai::native::open(
        std::path::Path::new(path),
        std::path::Path::new(tokenizer_path),
        device.clone(),
    )?;
    let context_window = loaded.model.context();
    let (model, tokenizer, eos_token_ids) = (loaded.model, loaded.tokenizer, loaded.eos);
    Ok(CandleThinker {
        model: CandleModel::Bonsai(model),
        tokenizer,
        device,
        model_label: format!("candle:qwen35-pq2:{label}@{path}"),
        architecture: "qwen35".into(),
        context_window,
        temperature: config.temperature,
        top_p: config.top_p,
        max_tokens: config.max_tokens.max(1),
        repeat_penalty: config.candle_repeat_penalty.max(1.0),
        repeat_last_n: config.candle_repeat_last_n.max(1),
        seed: config.candle_seed,
        request_index: 0,
        eos_token_ids,
        cached_tokens: Vec::new(),
    })
}
