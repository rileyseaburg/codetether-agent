//! Event-handler API exports, independent of module registration.

pub(crate) use super::event_dispatch::handle_event;
pub use super::mouse::handle_mouse_event;
pub use super::paste::handle_paste_event;
pub(crate) use super::voice::drain_voice_transcription;
