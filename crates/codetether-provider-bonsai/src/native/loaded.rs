//! Validated checkpoint loading, independent of agents and cognition sessions.
use anyhow::{Context, Result, ensure};
use candle_core::Device;
use std::{collections::HashSet, fs::File, io::BufReader, path::Path};
use tokenizers::Tokenizer;
/// Validated model, matching tokenizer, and EOS set for native generation.
///
/// Construct this bundle with [open].
pub struct Loaded {
    /// Native decoder state.
    pub model: super::Model,
    /// Tokenizer validated against the checkpoint vocabulary.
    pub tokenizer: Tokenizer,
    pub(crate) device: Device,
    /// Token IDs ending generation.
    pub eos: HashSet<u32>,
}
/// Load a validated checkpoint on the supplied CUDA device.
///
/// # Arguments
/// * `model` - Packed GGUF checkpoint path.
/// * `tokenizer` - Matching tokenizer JSON path.
/// * `device` - Selected CUDA device.
/// # Returns
/// The model, tokenizer, and EOS token IDs.
/// # Errors
/// Returns device, I/O, checkpoint contract, tokenizer, or tensor-load errors.
pub fn open(model: &Path, tokenizer: &Path, device: Device) -> Result<Loaded> {
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
