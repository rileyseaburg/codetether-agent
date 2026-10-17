//! State held by the in-process Candle inference runtime.

use candle_core::Device;
use std::collections::HashSet;
use tokenizers::Tokenizer;

use super::candle_model::CandleModel;

#[path = "candle_runtime/identity.rs"]
mod identity;
pub(super) use identity::NativeModelIdentity;

/// A loaded Candle model plus its sampling and KV-cache state.
///
/// Not `Sync`: callers serialize access behind a mutex.
///
/// # Examples
/// ```no_run
/// use codetether_cognition_native::{CandleThinker, ThinkerConfig};
/// # fn example(config: &ThinkerConfig) -> anyhow::Result<()> {
/// let _runtime = CandleThinker::new(config)?;
/// # Ok(())
/// # }
/// ```
pub struct CandleThinker {
    pub(super) model: CandleModel,
    pub(super) tokenizer: Tokenizer,
    pub(super) device: Device,
    pub(super) model_label: String,
    pub(crate) identity: NativeModelIdentity,
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

impl CandleThinker {
    /// Harness model identity captured when this runtime was loaded.
    pub fn identity_model(&self) -> &str {
        self.identity.model()
    }
}
