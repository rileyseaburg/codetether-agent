//! Voice-native pair programming — STT, command routing, TTS responses.

pub use codetether_voice_core::command_router;
pub use codetether_voice_core::dictation;
pub use codetether_voice_core::ssh_env;
pub use codetether_voice_core::stt_catalog;
pub mod transport;
