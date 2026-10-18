//! Frontend service selection for chat prompt submission.
#[cfg(test)]
#[path = "chat_submit_queue_tests.rs"]
mod queue_tests;
codetether_chat_submit::declare_frontend_chat_submit!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::provider::ProviderRegistry,
    crate::tui::worker_bridge::TuiWorkerBridge,
    crate::tui::app::session_runtime::TuiSessionHandle;
    crate::tui::app::input::goal_answer::commands::intercept,
    crate::tui::app::input::goal_answer::intercept,
    crate::tui::app::input::approval_feedback::submit;
    crate::tui::app::input::chat_submit_history::capture,
    crate::tui::app::managed_agent::chat::route,
    crate::tui::app::input::mention_route::route_prompt,
    crate::tui::app::input::continue_command::run,
    crate::tui::app::input::chat_submit_slash::run;
    crate::tui::app::input::chat_steer::submit,
    crate::tui::app::input::chat_submit_finish::finish_submit,
);
