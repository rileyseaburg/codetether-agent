//! Compact header encoding with distinct live and legacy metadata rules.
use crate::HeaderView;
use anyhow::Result;

/// Encode a live header after workspace and provenance normalization.
/// # Arguments
/// `session` is a borrowed view of the native header fields.
/// # Returns
/// Compact JSON with empty history arrays and no summary payload.
/// # Errors
/// Returns JSON serialization failures.
/// # Examples
/// ```rust
/// # fn example(view: codetether_session_codec::HeaderView<'_>) -> anyhow::Result<()> {
/// let header = codetether_session_codec::encode_header(view)?;
/// assert!(!header.is_empty());
/// # Ok(()) }
/// ```
pub fn encode_header(session: HeaderView<'_>) -> Result<String> {
    let mut metadata = (*session.metadata).clone();
    if let Some(directory) = &mut metadata.directory {
        *directory = directory
            .canonicalize()
            .unwrap_or_else(|_| directory.clone());
    }
    if let Some(identity) = codetether_provenance::runtime_agent_identity()
        && let Some(provenance) = metadata.provenance.as_mut()
    {
        provenance.identity.agent_identity_id = Some(identity);
    }
    super::header_write::write(&session, &metadata)
}

/// Encode a legacy header without changing its metadata.
/// # Arguments
/// `session` is a borrowed view of the native header fields.
/// # Returns
/// Compact JSON with empty history arrays and no summary payload.
/// # Errors
/// Returns JSON serialization failures.
/// # Examples
/// ```rust
/// # fn example(view: codetether_session_codec::HeaderView<'_>) -> anyhow::Result<()> {
/// let header = codetether_session_codec::legacy_header(view)?;
/// assert!(!header.is_empty());
/// # Ok(()) }
/// ```
pub fn legacy_header(session: HeaderView<'_>) -> Result<String> {
    super::header_write::write(&session, session.metadata)
}

/// Decode a stored header into the caller's native session type.
/// # Arguments
/// `header` contains the serialized small session record.
/// # Returns
/// The deserialized value, with defaults supplied by its own serde contract.
/// # Errors
/// Returns JSON parsing or target-shape validation errors.
/// # Examples
/// ```rust
/// let value: serde_json::Value = codetether_session_codec::decode_header("{}").unwrap();
/// assert!(value.is_object());
/// ```
pub fn decode_header<T: serde::de::DeserializeOwned>(header: &str) -> Result<T> {
    let session = serde_json::from_str(header)?;
    Ok(session)
}
