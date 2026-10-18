//! Release of paused approval requests after switching to full access.

use super::View;
use codetether_approval_queue as approval_queue;

/// Release pending requests in visible queue order.
/// # Arguments
/// * `view` — Frontend status, input and presentation bindings.
/// # Returns
/// Number of released requests, or an error when release makes no progress.
/// # Errors
/// Returns the command/status error if a pending request cannot be released.
/// # Examples
/// ```no_run
/// # fn example(view: &mut impl codetether_approval_view::command::View) {
/// let _ = codetether_approval_view::command::release_all(view);
/// # }
/// ```
pub fn all(view: &mut impl View) -> Result<usize, String> {
    let mut released = 0;
    loop {
        let before = target();
        let Some(before) = before else {
            break;
        };
        if !super::run(view, "/approve") {
            return Err("approval command unavailable".into());
        }
        if target().as_ref() == Some(&before) {
            return Err(view.status().to_string());
        }
        released += 1;
    }
    Ok(released)
}

fn target() -> Option<String> {
    approval_queue::active_id()
        .or_else(codetether_approval_core::interaction::pending_query::latest_id)
}
