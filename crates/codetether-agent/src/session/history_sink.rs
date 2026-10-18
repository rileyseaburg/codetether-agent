//! Shared durable history configuration, encoding, uploads and pointer reads.
pub use codetether_history_sink::{
    HistorySinkConfig, PointerHandle, encode_jsonl_delta, resolve_pointer, upload_full_history,
};
