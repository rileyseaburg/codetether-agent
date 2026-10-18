//! Frontend view selection for shared approval commands.
codetether_approval_view::declare_frontend_approval_command!(
    crate::tui::app::state::App,
    crate::tui::chat::message::ChatMessage,
    crate::tui::chat::message::MessageType,
);
pub(crate) use release::all as release_all;

#[cfg(test)]
#[path = "approval_command_order_support.rs"]
mod order_support;
#[cfg(test)]
#[path = "approval_command_order_tests.rs"]
mod order_tests;
#[cfg(test)]
#[path = "approval_command_queue_tests.rs"]
mod queue_tests;
#[path = "approval_command_release.rs"]
mod release;
#[cfg(test)]
#[path = "approval_command_test_support.rs"]
mod test_support;

#[cfg(test)]
#[path = "approval_command_navigation_tests.rs"]
mod navigation_tests;
