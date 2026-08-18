//! State held by the in-process Candle inference runtime.

use candle_core::Device;
use std::collections::HashSet;
use tokenizers::Tokenizer;

use super::candle_model::CandleModel;

/// A loaded Candle model plus its sampling and KV-cache state.
///
/// Not `Sync`: callers serialize access behind a mutex.
pub(crate) struct CandleThinker {
    pub(super) model: CandleModel,
    pub(super) tokenizer: Tokenizer,
    pub(super) device: Device,
    pub(super) model_label: String,
    pub(super) architecture: String,
    pub(super) context_window: usize,
    pub(super) temperature: f32,
    pub(super) top_p: Option<f32>,
    pub(super) max_tokens: usize,
    pub(super) repeat_penalty: f32,
    pub(super) repeat_last_n: usize,
    pub(super) seed: u64,
    pub(super) request_index: u64,
    pub(super) eos_token_ids: HashSet<u32>,
    pub(super) cached_tokens: Vec<u32>,
}
