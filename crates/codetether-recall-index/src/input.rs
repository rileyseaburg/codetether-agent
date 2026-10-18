//! Borrowed transcript and provenance inputs for projection construction.
use chrono::{DateTime, Utc};
use codetether_provider_core::Message;
use std::path::PathBuf;
/// Session fields needed to build an index without storage access.
/// # Examples
/// ```
/// use codetether_recall_index::{project, ProjectionInput};
/// let input = ProjectionInput { messages: &[], session_id: "id", title: None,
///     workspace: "/tmp".into(), updated_at: chrono::Utc::now(),
///     message_offset: 0, message_count: 0 };
/// assert!(project(input).documents.is_empty());
/// ```
pub struct ProjectionInput<'a> {
    /// Loaded transcript window.
    pub messages: &'a [Message],
    /// Source session identity.
    pub session_id: &'a str,
    /// Optional display title.
    pub title: Option<&'a str>,
    /// Canonical workspace selected by the caller.
    pub workspace: PathBuf,
    /// Snapshot modification time.
    pub updated_at: DateTime<Utc>,
    /// Absolute index of the first loaded message.
    pub message_offset: usize,
    /// Total transcript count, including unloaded messages.
    pub message_count: usize,
}
