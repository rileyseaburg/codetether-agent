//! Runtime-aware serialization of a borrowed session snapshot.
use anyhow::Result;
/// Serialize a full snapshot while borrowing its transcript and summary data.
/// # Arguments
/// `session` supplies borrowed fields and the existing page sidecar.
/// # Returns
/// JSON bytes with normalized metadata and aligned page classifications.
/// # Errors
/// Returns JSON serialization failures.
/// # Examples
/// ```rust
/// # fn example(view: codetether_session_codec::SnapshotView<'_>) -> anyhow::Result<()> {
/// let bytes = codetether_session_codec::serialize(view)?;
/// assert!(!bytes.is_empty());
/// # Ok(()) }
/// ```
pub fn serialize(session: crate::SnapshotView<'_>) -> Result<Vec<u8>> {
    let snapshot = crate::snapshot::Snapshot::from_view(session);
    let encode = || serde_json::to_vec(&snapshot).map_err(Into::into);
    match tokio::runtime::Handle::try_current().map(|handle| handle.runtime_flavor()) {
        Ok(tokio::runtime::RuntimeFlavor::MultiThread) => tokio::task::block_in_place(encode),
        Ok(tokio::runtime::RuntimeFlavor::CurrentThread) | Err(_) => encode(),
        _ => encode(),
    }
}
