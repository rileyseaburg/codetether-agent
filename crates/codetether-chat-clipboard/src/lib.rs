//! Chat clipboard copy and paste workflows.
//!
//! [handle_copy_reply] selects the latest assistant response;
//! [handle_copy_transcript] exports the conversation. [handle_clipboard_paste]
//! prefers normalized text and falls back to image attachments. A [Host]
//! supplies frontend state and the application's existing text-paste path.
mod clipboard;
mod copy_reply;
mod copy_transcript;
mod host;
pub use clipboard::handle_clipboard_paste;
pub use copy_reply::handle_copy_reply;
pub use copy_transcript::handle_copy_transcript;
pub use host::Host;
