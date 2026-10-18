//! Foreground RLM compaction with deterministic recovery.
//!
//! [context_summary] chooses a capable model from the caller's provider,
//! invokes the RLM router, and falls back to chunk compression on failure.
//! It does not access the application session or provider registry.
mod model;
mod selection;
mod summary;
pub use summary::context_summary;
#[cfg(test)]
mod tests;
