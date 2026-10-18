//! Buffer-level context compression independent of native session state.
//!
//! [CompressContext] owns the RLM settings and tracing identity for a pass.
//! [compress_messages_keep_last] preserves the active tail while summarizing
//! its prefix. [compress_last_message_if_oversized] bounds a large final message.
mod context;
pub mod experimental;
mod install;
mod keep_last;
mod oversized;
mod replacement;
mod text;
pub use context::CompressContext;
pub use keep_last::compress_messages_keep_last;
pub use oversized::compress_last_message_if_oversized;
