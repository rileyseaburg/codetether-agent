//! Serializable location of a full history object or bounded byte range.
use crate::HistorySinkConfig;
use serde::{Deserialize, Serialize};

/// Stable locator for a single `ResidencyLevel::Pointer` page's
/// backing bytes.
///
/// The Phase B incremental derivation can degrade an `Evidence` page
/// (or any page whose degradation path permits it) to `Pointer` and
/// hand the corresponding [`PointerHandle`] to
/// [`crate::resolve_pointer`] on demand. Keeping the handle pure data makes
/// it serialisable into the page sidecar without dragging the MinIO
/// client along.
///
/// # Examples
///
/// ```rust
/// use codetether_history_sink::PointerHandle;
///
/// let handle = PointerHandle {
///     bucket: "codetether".to_string(),
///     key: "history/abc-123/history.jsonl".to_string(),
///     byte_range: Some((0, 512)),
/// };
/// assert_eq!(handle.byte_range, Some((0, 512)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PointerHandle {
    /// S3 bucket name.
    pub bucket: String,
    /// Full object key.
    pub key: String,
    /// Optional half-open `(start, end)` byte range. When `None`, the
    /// full object is fetched.
    #[serde(default)]
    pub byte_range: Option<(u64, u64)>,
}

impl PointerHandle {
    /// Construct a handle that points at the whole history object for
    /// `session_id` under the sink's configured prefix.
    pub fn for_session(config: &HistorySinkConfig, session_id: &str) -> Self {
        Self {
            bucket: config.bucket.clone(),
            key: config.object_key(session_id),
            byte_range: None,
        }
    }
}
