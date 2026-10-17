//! Voice input, microphone capture, PCM resampling and WAV encoding.
//!
//! [VoiceInputTool] dispatches recording and transcription. [recorder] and
//! [encoder] also expose capture and WAV conversion to interactive applications.
//!
//! Provides a single `record_then_transcribe` action that captures audio
//! from the default microphone, encodes to WAV, and sends it to the
//! Voice API `/transcribe` endpoint for speech-to-text.

mod actions;
pub(crate) mod client;
mod device;
mod downmix;
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
#[cfg(target_os = "linux")]
mod stderr_redirect;
mod tool_impl;

pub use tool_impl::VoiceInputTool;
