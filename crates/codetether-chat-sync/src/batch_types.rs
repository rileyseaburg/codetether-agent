//! Metadata describing a successfully uploaded archive batch.

/// Metadata for a completed archive upload.
/// # Examples
/// ```
/// let batch = codetether_chat_sync::batch_upload::ChatSyncBatch { bytes: 10, records: 1, object_key: "batch.jsonl".into(), next_offset: 10 };
/// assert_eq!(batch.records, 1);
/// ```
pub struct ChatSyncBatch {
    /// Bytes uploaded.
    pub bytes: u64,
    /// Newline-delimited records counted.
    pub records: usize,
    /// Uploaded object key.
    pub object_key: String,
    /// Next archive byte offset.
    pub next_offset: u64,
}
