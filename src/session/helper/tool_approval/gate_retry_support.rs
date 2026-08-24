pub(super) struct QueueGuard;

impl QueueGuard {
    pub(super) fn new() -> Self {
        crate::tui::app::state::approval_queue::reset();
        Self
    }
}

impl Drop for QueueGuard {
    fn drop(&mut self) {
        crate::tui::app::state::approval_queue::reset();
    }
}
