//! Borrowed native identity and metadata fields used in compact headers.
use chrono::{DateTime, Utc};
use codetether_provider_core::Usage;
use codetether_session_metadata::SessionMetadata;
/// Native fields borrowed by header and snapshot encoders.
/// # Examples
/// ```rust
/// let metadata = codetether_session_metadata::SessionMetadata::default();
/// let now = chrono::Utc::now();
/// let usage = codetether_provider_core::Usage::default();
/// let header = codetether_session_codec::HeaderView {
///     id: "session-123", title: &None, created_at: &now, updated_at: &now,
///     metadata: &metadata, agent: "default", usage: &usage,
/// };
/// assert_eq!(header.id, "session-123");
/// ```
pub struct HeaderView<'a> {
    /// Durable session identity.
    pub id: &'a str,
    /// Optional session title.
    pub title: &'a Option<String>,
    /// Creation timestamp.
    pub created_at: &'a DateTime<Utc>,
    /// Last update timestamp.
    pub updated_at: &'a DateTime<Utc>,
    /// Session settings; normalization always works on a clone.
    pub metadata: &'a SessionMetadata,
    /// Agent persona name.
    pub agent: &'a str,
    /// Recorded provider usage.
    pub usage: &'a Usage,
}
