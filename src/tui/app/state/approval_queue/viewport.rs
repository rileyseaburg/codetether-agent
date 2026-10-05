//! Scroll bounds shared by approval rendering and input handling.

/// Cache the current row limit on the request whose details were rendered.
pub(crate) fn set_scroll_limit(id: &str, limit: u16) {
    let mut guard = super::queue().lock().expect("approval queue lock");
    if let Some(item) = guard.iter_mut().find(|item| item.id == id) {
        item.scroll_limit = limit;
    }
}
