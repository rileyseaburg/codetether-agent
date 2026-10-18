//! Frontend resource and service selection for goal-command input.
#[cfg(test)]
use codetether_goal_command::input as pending;
codetether_goal_command::declare_frontend_goal_input!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::tui::app::session_runtime::TuiSessionHandle,
    crate::provider::ProviderRegistry,
    crate::tui::worker_bridge::TuiWorkerBridge,
    crate::tui::chat::message::ChatMessage,
    crate::tui::chat::message::MessageType,
    codetether_goal_core::TaskState;
    crate::tui::app::commands::goal::handle,
    crate::session::tasks::runtime::answer_review::read,
    crate::tui::app::input::goal_answer::editor::open,
    crate::tui::app::input::sessions::goal_autostart::resume,
);
#[cfg(test)]
mod tests;
