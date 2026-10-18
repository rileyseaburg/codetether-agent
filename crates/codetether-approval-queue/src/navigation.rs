//! Explicit browsing of pending requests without resolving them.

/// Move the displayed request; decisions still target the displayed queue head.
/// Rotate the displayed request forward or backward when multiple requests exist.
/// # Arguments
/// * `forward` - Rotate left when true, right otherwise.
/// # Returns
/// Updates queue order in place.
/// # Panics
/// Panics if the process-wide queue mutex was poisoned.
/// # Examples
/// ```no_run
/// codetether_approval_queue::cycle(true);
/// ```
pub fn cycle(forward: bool) {
    let mut guard = super::queue().lock().expect("approval queue lock");
    if guard.len() > 1 {
        if forward {
            guard.rotate_left(1);
        } else {
            guard.rotate_right(1);
        }
    }
}
