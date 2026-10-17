//! Speech-model metadata and decoder support contracts.

/// Whether an in-process candle decoder exists for this architecture.
///
/// # Examples
/// ```
/// use codetether_voice_core::stt_catalog::DecoderSupport;
/// match DecoderSupport::MissingDecoder {
///     DecoderSupport::MissingDecoder => {},
///     DecoderSupport::Available => panic!("decoder not expected"),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecoderSupport {
    /// A candle loader exists and the model can run in-process.
    Available,
    /// Weights are usable but no candle decoder exists yet.
    MissingDecoder,
}

/// Metadata for a catalogued speech-to-text GGUF.
///
/// # Examples
/// ```
/// use codetether_voice_core::stt_catalog::{DecoderSupport, SttModel};
/// let model = SttModel { id: "example", arch: "example", sample_rate: 16_000,
///     streaming: false, support: DecoderSupport::MissingDecoder };
/// assert_eq!(model.sample_rate, 16_000);
/// ```
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
