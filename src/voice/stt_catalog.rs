//! Registry of local speech-to-text GGUF models.
//!
//! Separate from [`crate::provider::local_catalog`], which lists text
//! models candle can decode today. ASR GGUFs declare their own
//! `general.architecture` (for example `parakeet`) and need an encoder
//! that candle 0.11 does not ship, so entries here record readiness
//! explicitly instead of implying they will load.

/// Whether an in-process candle decoder exists for this architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecoderSupport {
    /// A candle loader exists and the model can run in-process.
    Available,
    /// Weights are usable but no candle decoder exists yet.
    MissingDecoder,
}

/// A speech-to-text GGUF available on local disk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SttModel {
    /// Selectable id.
    pub id: &'static str,
    /// GGUF `general.architecture` read from the file header.
    pub arch: &'static str,
    /// Microphone sample rate the frontend expects, in Hz.
    pub sample_rate: u32,
    /// Whether streaming (incremental) decoding is supported.
    pub streaming: bool,
    /// In-process decoder readiness.
    pub support: DecoderSupport,
}

/// NVIDIA ASR models, with support state verified against candle 0.11.
pub const STT_MODELS: &[SttModel] = &[SttModel {
    id: "nemotron-speech-streaming-en-0.6b",
    arch: "parakeet",
    sample_rate: 16_000,
    streaming: true,
    support: DecoderSupport::MissingDecoder,
}];

/// Look up a speech model by id.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::voice::stt_catalog::{DecoderSupport, find};
///
/// let m = find("nemotron-speech-streaming-en-0.6b").unwrap();
/// assert_eq!(m.arch, "parakeet");
/// assert_eq!(m.support, DecoderSupport::MissingDecoder);
/// ```
pub fn find(id: &str) -> Option<&'static SttModel> {
    STT_MODELS.iter().find(|model| model.id == id)
}

/// Ids that can actually run in-process right now.
pub fn runnable() -> Vec<&'static str> {
    STT_MODELS
        .iter()
        .filter(|m| m.support == DecoderSupport::Available)
        .map(|m| m.id)
        .collect()
}

#[cfg(test)]
#[path = "stt_catalog_tests.rs"]
mod tests;
