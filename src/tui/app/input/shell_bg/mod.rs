//! Frontend state selection for completed shell output.
codetether_chat_shell::declare_frontend_shell_view!(
    crate::tui::app::state::App,
    codetether_chat_model::ChatMessage,
);
