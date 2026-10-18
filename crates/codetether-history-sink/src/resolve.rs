//! Resolve an object pointer with the original range and fault semantics.
use crate::{HistorySinkConfig, PointerHandle};
use codetether_context_policy::fault::Fault;

/// Fetch the bytes referenced by a [`PointerHandle`] from the MinIO
/// sink.
///
/// Returns a typed [`Fault`] so callers get the same reason-code
/// surface ClawVM §3 prescribes for context-construction failures
/// (*silent* empty returns are forbidden).
///
/// # Errors
///
/// * [`Fault::BackendError`] when the MinIO client build or the GET
///   itself fails.
/// * [`Fault::NoMatch`] when the object is missing but the backend
///   responded cleanly.
///
/// # Range semantics
///
/// When `byte_range` is `Some((start, end))`, the returned slice is
/// truncated to that half-open range within the full object body.
/// Ranges that fall off the end are clamped to `object.len()`.
/// # Arguments
/// `config` supplies backend credentials; `handle` selects the object and range.
/// # Returns
/// Backend response bytes for the requested object or range.
/// # Examples
/// ```rust,no_run
/// # async fn example(config: &codetether_history_sink::HistorySinkConfig, handle: &codetether_history_sink::PointerHandle) {
/// let result = codetether_history_sink::resolve_pointer(config, handle).await;
/// # }
/// ```
pub async fn resolve_pointer(
    config: &HistorySinkConfig,
    handle: &PointerHandle,
) -> Result<Vec<u8>, Fault> {
    let client = crate::client::build_client(config).map_err(|e| Fault::BackendError {
        reason: format!("minio client build failed: {e}"),
    })?;

    // Use HTTP Range requests when the handle specifies a byte range so
    // we don't download the whole history object just to slice out a
    // few KB. `offset` + `length` map directly to a `Range` header in
    // the minio 0.3 builder.
    let (offset, length) = match handle.byte_range {
        Some((start, end)) => {
            let len = end.saturating_sub(start);
            if len == 0 {
                return Err(Fault::NoMatch);
            }
            (Some(start), Some(len))
        }
        None => (None, None),
    };

    crate::fetch::bytes(client, handle, offset, length).await
}
