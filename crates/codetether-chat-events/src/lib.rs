//! Chat stream, retry and usage-event presentation.
//!
//! [text] updates streamed and completed messages through [StreamView].
//! [usage] records completion metrics through [UsageView]. Both event handlers
//! return unrelated events unchanged for the surrounding event pipeline.
mod attach;
mod bounded;
mod dispatch;
pub mod retry;
mod stream_view;
pub mod text;
pub mod usage;
mod usage_view;
pub use attach::attach_usage_to_last_completion_message;
pub use dispatch::handle_event as handle_text_event;
pub use stream_view::StreamView;
pub use usage_view::UsageView;

/// Tool-call messages, timing and structured completion status.
pub mod tools;
