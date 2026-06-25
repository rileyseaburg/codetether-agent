//! Voice Input Tool — record microphone audio, transcribe via Whisper API.
//!
//! Provides a single `record_then_transcribe` action that captures audio
//! from the default microphone, encodes to WAV, and sends it to the
//! Voice API `/transcribe` endpoint for speech-to-text.

mod actions;
pub(crate) mod client;
mod device;
pub mod encoder;
mod input_builder;
mod input_config;
mod input_stream;
mod params;
pub mod recorder;
mod resampler;
#[cfg(test)]
mod resampler_tests;
mod schema;
pub(crate) mod stderr_guard;
mod tool_impl;

pub use tool_impl::VoiceInputTool;
