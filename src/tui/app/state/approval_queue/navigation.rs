//! Explicit browsing of pending requests without resolving them.

/// Move the displayed request; decisions still target the displayed queue head.
pub(crate) fn cycle(forward: bool) {
    let mut guard = super::queue().lock().expect("approval queue lock");
    if guard.len() > 1 {
        if forward {
            guard.rotate_left(1);
        } else {
            guard.rotate_right(1);
        }
    }
}
