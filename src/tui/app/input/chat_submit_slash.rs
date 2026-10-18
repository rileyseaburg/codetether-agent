//! Frontend command modules and callback selection for chat intake.
#[path = "mux_command/mod.rs"]
mod mux_command;
#[path = "shell/mod.rs"]
mod shell;
#[path = "slash_no_session.rs"]
mod slash_no_session;
#[cfg(test)]
#[path = "slash_no_session_tests.rs"]
mod slash_no_session_tests;

codetether_slash_command::declare_frontend_command_intake!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::provider::ProviderRegistry,
    crate::session::Session;
    crate::tui::app::input::chat_submit_slash::shell::run,
    crate::tui::app::input::approval_command::run,
    crate::tui::app::input::chat_submit_slash::mux_command::run,
    crate::tui::app::input::chat_submit_slash::slash_no_session::run;
    crate::tui::app::input::codex_parity_command::run,
    crate::tui::app::commands::handle_slash_command,
);
