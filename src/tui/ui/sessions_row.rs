//! Formatting for a single session-picker list row.

use crate::session::SessionSummary;

/// Build the display string for one session row.
///
/// Includes a short session-id prefix so users can see what to
/// type when filtering the picker by ID.
///
/// # Examples
///
/// ```ignore
/// let row = session_row_summary(&summary, true);
/// ```
pub fn session_row_summary(session: &SessionSummary, is_active: bool) -> String {
    let title = session
        .title
        .clone()
        .unwrap_or_else(|| "Untitled session".to_string());
    let active_marker = if is_active { " ●" } else { "" };
    format!(
        "{}  {}{}  •  {} msgs  •  {}",
        session.id.get(..8).unwrap_or(session.id.as_str()),
        title,
        active_marker,
        session.message_count,
        session.updated_at.format("%Y-%m-%d %H:%M")
    )
}
