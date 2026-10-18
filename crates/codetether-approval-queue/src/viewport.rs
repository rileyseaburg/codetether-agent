//! Scroll bounds shared by approval rendering and input handling.

/// Cache the current row limit on the request whose details were rendered.
/// Store the rendered-row limit for a still-queued request.
/// # Arguments
/// * `id` - Request whose preview was rendered.
/// * `limit` - Last scroll row.
/// # Returns
/// Updates a matching snapshot when present.
/// # Panics
/// Panics if the process-wide queue mutex was poisoned.
/// # Examples
/// ```no_run
/// codetether_approval_queue::set_scroll_limit("id", 20);
/// ```
pub fn set_scroll_limit(id: &str, limit: u16) {
    let mut guard = super::queue().lock().expect("approval queue lock");
    if let Some(item) = guard.iter_mut().find(|item| item.id == id) {
        item.scroll_limit = limit;
    }
}
