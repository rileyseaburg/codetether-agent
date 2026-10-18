//! Status glyph + elapsed-time formatting for swarm subtask rows.

use crate::SubTaskInfo;
use codetether_swarm_core::SubTaskStatus;
use ratatui::style::Color;

/// Icon and color for a subtask status. Avoids `DarkGray` (poor contrast) in
/// favor of `Gray` for WCAG 1.4.3 legibility on dark terminals.
/// # Arguments
/// * `status` - Current subtask state.
/// # Returns
/// The display glyph and its terminal color.
/// # Examples
/// ```
/// use codetether_swarm_core::SubTaskStatus;
/// let (glyph, _) = codetether_swarm_view::swarm_view_fmt::status_glyph(SubTaskStatus::Running);
/// assert_eq!(glyph, "●");
/// ```
pub fn status_glyph(status: SubTaskStatus) -> (&'static str, Color) {
    match status {
        SubTaskStatus::Pending => ("○", Color::Gray),
        SubTaskStatus::Blocked => ("⊘", Color::Yellow),
        SubTaskStatus::Running => ("●", Color::Cyan),
        SubTaskStatus::Completed => ("✓", Color::Green),
        SubTaskStatus::Failed => ("✗", Color::Red),
        SubTaskStatus::Cancelled => ("⊗", Color::Gray),
        SubTaskStatus::TimedOut => ("⏱", Color::Red),
    }
}

/// Format the agent's running/finished wall-clock time, e.g. `12s` or `2m03s`.
///
/// Returns the frozen `elapsed_secs` once set, otherwise the live duration
/// since `started_at` while the agent is still running.
/// # Arguments
/// * `task` - Task containing start time and optional frozen duration.
/// # Returns
/// A seconds/minutes label, or None when no duration is available.
/// # Examples
/// ```no_run
/// # fn example(task: &codetether_swarm_view::SubTaskInfo) {
/// let label = codetether_swarm_view::swarm_view_fmt::elapsed_label(task);
/// # }
/// ```
pub fn elapsed_label(task: &SubTaskInfo) -> Option<String> {
    let secs = task.elapsed_secs.or_else(|| {
        (task.status == SubTaskStatus::Running)
            .then(|| task.started_at.map(|s| s.elapsed().as_secs()))
            .flatten()
    })?;
    Some(if secs >= 60 {
        format!("{}m{:02}s", secs / 60, secs % 60)
    } else {
        format!("{secs}s")
    })
}

#[cfg(test)]
#[path = "swarm_view_fmt_tests.rs"]
mod tests;
