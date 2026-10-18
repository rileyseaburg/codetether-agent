//! Frontend type selection for held-answer delivery.
codetether_goal_answer::declare_frontend_answer_delivery!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::provider::ProviderRegistry,
);
#[cfg(test)]
#[path = "answer_tests.rs"]
mod tests;
