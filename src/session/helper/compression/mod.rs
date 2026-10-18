//! Native session wrappers over shared buffer compression and budget enforcement.
use codetether_context_compression as context;
use codetether_context_policy::compression::summary as compression_last_message;
use codetether_context_policy::compression::summary as compression_summary;
use codetether_rlm_foreground as compression_defer;
pub(crate) mod dropped_toc;
mod enforce;
mod enforce_session;
use codetether_context_compression as keep_last;
mod keep_last_session;
mod oversized;

#[cfg(test)]
mod tests_context;

pub(crate) use context::CompressContext;
pub(crate) use enforce::enforce_on_messages;
pub use enforce_session::enforce_context_window;
pub(crate) use keep_last::compress_messages_keep_last;
pub use keep_last_session::compress_history_keep_last;
pub(crate) use oversized::compress_last_message_if_oversized;
