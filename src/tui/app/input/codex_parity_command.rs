//! Frontend selection of session utility handlers.
#[path = "access_mode_command.rs"]
mod access_mode_command;
#[path = "codex_parity_copy.rs"]
mod copy;
#[path = "codex_parity_diff.rs"]
mod diff;
#[path = "codex_parity_status.rs"]
mod status;
#[cfg(test)]
#[path = "codex_parity_command_tests.rs"]
mod tests;

codetether_slash_command::declare_frontend_utility!(
    crate::tui::app::state::App,
    crate::session::Session,
    crate::provider::ProviderRegistry,
    crate::tui::chat::message::ChatMessage,
    crate::tui::chat::message::MessageType;
    crate::tui::app::input::codex_parity_command::access_mode_command::run,
    crate::tui::app::input::codex_parity_command::status::show,
    crate::tui::app::input::codex_parity_command::diff::show,
    crate::tui::app::input::codex_parity_command::copy::run,
    crate::tui::app::commands::handle_slash_command,
);
