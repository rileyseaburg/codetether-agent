//! Built-in speech model metadata.
use super::{DecoderSupport, SttModel};

/// NVIDIA ASR models, with support state verified against candle 0.11.
pub const STT_MODELS: &[SttModel] = &[SttModel {
    id: "nemotron-speech-streaming-en-0.6b",
    arch: "parakeet",
    sample_rate: 16_000,
    streaming: true,
    support: DecoderSupport::MissingDecoder,
}];
