//! Frontend type selection for copy-command presentation.
codetether_chat_clipboard::declare_frontend_copy_command!(
    crate::tui::app::state::App,
    codetether_chat_model::ChatMessage,
);
#[cfg(test)]
#[path = "codex_parity_copy_tests.rs"]
mod tests;
