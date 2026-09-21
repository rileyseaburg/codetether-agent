//! Validated checkpoint loading, independent of agents and cognition sessions.
use anyhow::{Context, Result, ensure};
use candle_core::Device;
use std::{collections::HashSet, fs::File, io::BufReader, path::Path};
use tokenizers::Tokenizer;
pub(crate) struct Loaded {
    pub(crate) model: super::Model,
    pub(crate) tokenizer: Tokenizer,
    pub(crate) device: Device,
    pub(crate) eos: HashSet<u32>,
}
pub(crate) fn open(model: &Path, tokenizer: &Path, device: Device) -> Result<Loaded> {
    ensure!(
        device.is_cuda(),
        "Bonsai requires native CUDA; CPU fallback is disabled"
    );
    let mut reader = BufReader::new(File::open(model).context("Open Bonsai GGUF")?);
    let index = super::Index::read(&mut reader)?;
    super::validate(&index)?;
    let tokenizer = Tokenizer::from_file(tokenizer)
        .map_err(|error| anyhow::anyhow!("Load Bonsai tokenizer: {error}"))?;
    super::tokenizer::validate(&tokenizer, &index)?;
    let id = index
        .metadata
        .get("tokenizer.ggml.eos_token_id")
        .and_then(serde_json::Value::as_u64)
        .context("Missing Bonsai EOS")?;
    let mut eos = HashSet::from([u32::try_from(id)?]);
    for marker in ["<|im_end|>", "<|endoftext|>"] {
        if let Some(id) = tokenizer.token_to_id(marker) {
            eos.insert(id);
        }
    }
    let model = super::load::model(&index, &mut reader, &device)?;
    Ok(Loaded {
        model,
        tokenizer,
        device,
        eos,
    })
}
