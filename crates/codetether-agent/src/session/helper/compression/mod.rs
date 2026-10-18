//! Native session wrappers over shared buffer compression and budget enforcement.
use codetether_context_compression as context;
mod enforce;
mod enforce_session;
use codetether_context_compression as keep_last;
mod keep_last_session;

#[cfg(test)]
mod tests_context;

pub use enforce_session::enforce_context_window;
pub use keep_last_session::compress_history_keep_last;
