//! Background microphone recording and blocking speech transcription.
//!
//! [spawn_voice_worker] records and transcribes on a dedicated thread, then
//! publishes text to a caller-owned [VoiceSlot]. [max_duration] preserves the
//! environment limit. [transcribe_sync] also accepts already-encoded WAV audio.
mod config;
mod transcribe;
mod worker;
pub use config::max_duration;
pub use transcribe::transcribe_sync;
pub use worker::{VoiceSlot, spawn_voice_worker};

/// Older blocking drain retained for compatibility, separate from active polling.
pub mod compatibility;
mod controls;
mod drain;
mod input;
pub use controls::{Controls, toggle_recording};
pub use drain::drain_voice_transcription;
pub use input::Input;
mod state;
mod transcription_input;
pub use state::RecordingState;
pub use transcription_input::TranscriptionInput;
