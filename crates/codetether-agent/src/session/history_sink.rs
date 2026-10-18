//! Shared durable history configuration, encoding, uploads and pointer reads.
pub(crate) use codetether_history_sink::upload_encoded_history;
pub use codetether_history_sink::{
    HistorySinkConfig, PointerHandle, encode_jsonl_delta, resolve_pointer, upload_full_history,
};
