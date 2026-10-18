//! REST compatibility bridge for voice-session bus notifications.
//!
//! Creation/deletion publish through the caller-supplied bus. Response fields,
//! configured LiveKit URL and the static voice catalog retain the legacy API.
mod create;
mod delete;
mod get;
pub mod http;
mod list;
mod request;
pub use create::create_voice_session_rest;
pub use delete::delete_voice_session_rest;
pub use get::get_voice_session_rest;
pub use list::list_voices_rest;
pub use request::CreateVoiceSessionRequest;
