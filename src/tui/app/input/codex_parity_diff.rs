//! Frontend type selection for working-tree report presentation.
codetether_git_view::declare_frontend_diff_command!(
    crate::tui::app::state::App,
    crate::tui::chat::message::ChatMessage,
    crate::tui::chat::message::MessageType,
);
