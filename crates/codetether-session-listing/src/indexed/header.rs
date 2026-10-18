//! Project a validated application header into a session summary.
use codetether_session_metadata::SessionSummary;
use serde::de::DeserializeOwned;
/// Application header decoding and summary projection for indexed listings.
/// # Implementors
/// Implement on the same session type used by normal loading so malformed
/// headers retain that type's validation and defaults.
pub trait Header: DeserializeOwned + 'static {
    /// Consume the decoded header and use the store's durable message count.
    fn into_summary(self, message_count: usize) -> SessionSummary;
}
pub(super) fn decode<H: Header>(header: &str, count: usize) -> anyhow::Result<SessionSummary> {
    let session: H = codetether_session_codec::decode_header(header)?;
    Ok(session.into_summary(count))
}
