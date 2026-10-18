//! Native session wrappers over shared buffer compression and budget enforcement.
use codetether_context_compression as context;
use codetether_context_policy::compression::summary as compression_last_message;
use codetether_context_policy::compression::summary as compression_summary;
pub(crate) use codetether_context_policy::compression::summary::toc as dropped_toc;
use codetether_rlm_foreground as compression_defer;
mod enforce;
mod enforce_session;
use codetether_context_compression as keep_last;
mod keep_last_session;

#[cfg(test)]
mod tests_context;

pub(crate) use codetether_native_context::compress_last_message_if_oversized;
pub(crate) use context::CompressContext;
pub(crate) use enforce::enforce_on_messages;
pub use enforce_session::enforce_context_window;
pub(crate) use keep_last::compress_messages_keep_last;
pub use keep_last_session::compress_history_keep_last;
