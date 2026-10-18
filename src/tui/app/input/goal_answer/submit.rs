//! Frontend type selection for question interception.
codetether_goal_answer::declare_frontend_answer_submit!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::tui::app::session_runtime::TuiSessionHandle,
);
#[cfg(test)]
#[path = "submit_followup_tests.rs"]
mod followup_tests;
#[cfg(test)]
#[path = "submit_tests.rs"]
mod tests;
