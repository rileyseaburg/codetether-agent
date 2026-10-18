//! Session-title selection and compact display fallback.

/// Resolve and compact the active session title or identifier.
///
/// # Arguments
/// * `sessions` - Cached session summaries.
/// * `selected` - Selected index in that list.
/// * `session_id` - Active session identifier, if attached.
///
/// # Returns
/// A compact display label, or new when no session is attached.
///
/// # Examples
/// ```
/// assert_eq!(codetether_session_picker::session_label(&[], 0, None), "new");
/// ```
pub fn session_label(
    sessions: &[codetether_session_metadata::SessionSummary],
    selected: usize,
    session_id: Option<&str>,
) -> String {
    let Some(id) = session_id else {
        return "new".to_string();
    };
    let label = sessions
        .get(selected)
        .filter(|session| session.id == id)
        .map(codetether_session_metadata::SessionSummary::display_label)
        .unwrap_or_else(|| id.to_string());
    compact(&label)
}

fn compact(label: &str) -> String {
    if label.len() > 28 {
        format!(
            "{}…",
            codetether_util::truncate::truncate_bytes_safe(label, 27)
        )
    } else {
        label.to_string()
    }
}
