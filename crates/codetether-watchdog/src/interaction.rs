//! Pending user questions suspend stall detection and refresh activity.
use std::time::Instant;
/// Whether a session has an unanswered question.
/// # Arguments
/// Receives the optional current session identifier.
/// # Returns
/// True only when the question runtime reports a pending question.
/// # Examples
/// ```
/// assert!(!codetether_watchdog::interaction::waiting(None));
/// ```
pub fn waiting(session_id: Option<&str>) -> bool {
    session_id.is_some_and(|id| codetether_question::runtime::pending(id).is_some())
}
/// Apply early user-wait gates before probing provider inactivity.
/// # Arguments
/// Receives processing, notification, approval and session state.
/// # Returns
/// True when a stall check should be skipped; question lookup is last.
/// # Examples
/// ```
/// assert!(codetether_watchdog::interaction::skip_stall_check(false, false, false, None));
/// ```
pub fn skip_stall_check(
    processing: bool,
    notified: bool,
    approval_waiting: bool,
    session_id: Option<&str>,
) -> bool {
    !processing || notified || approval_waiting || waiting(session_id)
}
/// Refresh activity while the user has an unanswered question.
/// # Arguments
/// Receives session identity, redraw flag and the existing activity clock.
/// # Returns
/// Marks the display dirty and rearms inactivity timing only while waiting.
/// # Examples
/// ```
/// let mut redraw = false;
/// let mut last_event = None;
/// codetether_watchdog::interaction::tick(None, &mut redraw, &mut last_event);
/// assert!(!redraw && last_event.is_none());
/// ```
pub fn tick(session_id: Option<&str>, needs_redraw: &mut bool, last_event: &mut Option<Instant>) {
    if waiting(session_id) {
        *needs_redraw = true;
        *last_event = Some(Instant::now());
    }
}
