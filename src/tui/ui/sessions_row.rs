//! Formatting for a single session-picker list row.

use crate::session::SessionSummary;

mod relative_time;
#[cfg(test)]
mod tests;

use relative_time::relative_time;

/// Build the display string for one session row.
///
/// Layout: `<id8>  <title> [•active]  ·  <dir>  ·  <n> msgs  ·  <ago>`.
/// The project directory basename disambiguates sessions that share a
/// near-identical first-message title, and the relative age is easier to
/// scan than an absolute timestamp.
///
/// # Examples
///
/// ```ignore
/// let row = session_row_summary(&summary, true);
/// ```
pub fn session_row_summary(session: &SessionSummary, is_active: bool) -> String {
    let title = session
        .display_title()
        .unwrap_or_else(|| "Untitled session".to_string());
    let active_marker = if is_active { " ●" } else { "" };
    let dir = session
        .directory
        .as_deref()
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "~".to_string());
    format!(
        "{}  {}{}  ·  {}  ·  {} msgs  ·  {}",
        session.id.get(..8).unwrap_or(session.id.as_str()),
        title,
        active_marker,
        dir,
        session.message_count,
        relative_time(session.updated_at),
    )
}
