//! Bind the packed native Bonsai decoder to the existing Candle generation harness.
use super::super::{
    ThinkerConfig, candle_device, candle_eos, candle_model::CandleModel, candle_resolve,
    candle_runtime::CandleThinker,
};
use anyhow::{Context, Result};
use std::{fs::File, io::BufReader};
pub(super) fn load(config: &ThinkerConfig) -> Result<CandleThinker> {
    let (path, tokenizer_path) = candle_resolve::paths(config)?;
    let (device, label) = candle_device::select_candle_device(config)?;
    let mut reader = BufReader::new(File::open(path)?);
    let index = super::Index::read(&mut reader)?;
    super::validate(&index)?;
    let tokenizer = tokenizers::Tokenizer::from_file(tokenizer_path)
        .map_err(|_| anyhow::anyhow!("Cannot load Bonsai tokenizer"))?;
    super::tokenizer::validate(&tokenizer, &index)?;
    let eos = index
        .metadata
        .get("tokenizer.ggml.eos_token_id")
        .and_then(serde_json::Value::as_u64)
        .context("Missing Bonsai EOS token")?;
    let eos_token_ids = candle_eos::collect_eos_token_ids(&tokenizer, &[u32::try_from(eos)?]);
    let model = super::load::model(&index, &mut reader, &device)?;
    let context_window = model.context();
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
