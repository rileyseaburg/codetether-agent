//! Candle BERT embedding model loader.

use super::hf_download::ModelFiles;
use anyhow::{Context, Result};
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config, DTYPE};
use std::sync::Mutex;
use tokenizers::Tokenizer;

/// A loaded BERT model that produces sentence embeddings via mean pooling.
pub struct BertEmbedder {
    pub(super) model: BertModel,
    pub(super) tokenizer: Tokenizer,
    pub(super) device: Device,
    /// Serializes inference; candle tensors are not `Sync` across threads.
    pub(super) lock: Mutex<()>,
}

impl BertEmbedder {
    /// Load the model from downloaded `files`, using CUDA when available.
    pub fn load(files: &ModelFiles) -> Result<Self> {
        let device = Device::cuda_if_available(0).unwrap_or(Device::Cpu);
        let config: Config =
            serde_json::from_slice(&std::fs::read(&files.config).context("read bert config.json")?)
                .context("parse bert config.json")?;

        let tokenizer = Tokenizer::from_file(&files.tokenizer)
            .map_err(|e| anyhow::anyhow!("load tokenizer: {e}"))?;

        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(
                std::slice::from_ref(&files.weights),
                DTYPE,
                &device,
            )
            .context("mmap bert safetensors")?
        };
        let model = BertModel::load(vb, &config).context("load bert model")?;

        Ok(Self {
            model,
            tokenizer,
            device,
            lock: Mutex::new(()),
        })
    }
}

/// Encode `text` to input-id and token-type tensors of shape `[1, seq]`.
pub(super) fn encode(embedder: &BertEmbedder, text: &str) -> Result<(Tensor, Tensor)> {
    let encoding = embedder
        .tokenizer
        .encode(text, true)
        .map_err(|e| anyhow::anyhow!("tokenize: {e}"))?;
    let ids: Vec<u32> = encoding.get_ids().to_vec();
    let len = ids.len();
    let input_ids = Tensor::new(ids, &embedder.device)?.reshape((1, len))?;
    let token_type_ids = input_ids.zeros_like()?;
    Ok((input_ids, token_type_ids))
}
