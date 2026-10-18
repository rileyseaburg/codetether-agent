//! Frontend approval queue selection for shared reconciliation.
codetether_tui_tick::declare_frontend_approval!(
    crate::tui::app::state::App,
    crate::tui::app::state::approval_queue::remove_stale,
    crate::tui::app::state::approval_queue::len,
);
#[cfg(test)]
#[path = "approval_tests.rs"]
mod tests;
