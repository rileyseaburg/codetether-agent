//! Constructor for the Candle inference runtime.

use anyhow::{Context, Result, anyhow};
use candle_core::quantized::gguf_file;
use std::fs::File;
use std::io::BufReader;
use tokenizers::Tokenizer;

use super::candle_resolve::{DEFAULT_CONTEXT_WINDOW, architecture, paths};
use super::candle_runtime::CandleThinker;
use super::{ThinkerConfig, candle_device, candle_eos, candle_gguf, candle_load};

impl CandleThinker {
    /// Load a GGUF model and tokenizer described by `config`.
    ///
    /// # Errors
    ///
    /// Returns an error when required paths are unset, the device cannot be
    /// acquired, or the model/tokenizer fails to load.
    pub(crate) fn new(config: &ThinkerConfig) -> Result<Self> {
        let (model_path, tokenizer_path) = paths(config)?;
        let (device, device_label) = candle_device::select_candle_device(config)?;
        let mut reader = BufReader::new(
            File::open(model_path)
                .with_context(|| format!("failed to open candle model file at {model_path}"))?,
        );
        let content = gguf_file::Content::read(&mut reader)
            .with_context(|| format!("failed to parse gguf model metadata from {model_path}"))?;

        let arch = architecture(config, &content);
        let context_window =
            candle_gguf::detect_context_window(&content, &arch).unwrap_or(DEFAULT_CONTEXT_WINDOW);
        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .map_err(|e| anyhow!("failed to load tokenizer from {tokenizer_path}: {e}"))?;
        let eos_token_ids = candle_eos::collect_eos_token_ids(
            &tokenizer,
            &candle_gguf::extract_gguf_eos_ids(&content),
        );
        if eos_token_ids.is_empty() {
            tracing::warn!("No EOS tokens in tokenizer; generation stops on max token limit");
        }
        let model = candle_load::load(&arch, content, &mut reader, &device, model_path)?;

        Ok(Self {
            model,
            tokenizer,
            device,
            model_label: format!("candle:{arch}:{device_label}@{model_path}"),
            architecture: arch,
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
}
