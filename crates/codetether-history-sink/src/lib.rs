//! Durable JSONL history uploads and referenced object reads.
//!
//! [HistorySinkConfig] keeps the existing environment configuration and object
//! key layout. [encode_jsonl_delta] produces one line per message; upload helpers
//! write those bytes. [PointerHandle] and [resolve_pointer] read whole objects or
//! bounded ranges, preserving typed context faults on missing/failed reads.
mod client;
mod config;
mod config_env;
mod config_key;
mod encode;
mod fetch;
mod pointer;
mod resolve;
mod upload_encoded;
mod upload_full;
pub use config::HistorySinkConfig;
pub use encode::encode_jsonl_delta;
pub use pointer::PointerHandle;
pub use resolve::resolve_pointer;
pub use upload_encoded::encoded as upload_encoded_history;
pub use upload_full::upload_full_history;
#[cfg(test)]
mod tests;

/// Incremental archival from durable consumer snapshots.
pub mod archive;
