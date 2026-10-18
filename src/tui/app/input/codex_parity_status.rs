//! Frontend type selection for session status reporting.
codetether_chat_status::declare_frontend_status_command!(
    crate::tui::app::state::App,
    crate::session::Session,
    crate::tui::chat::message::ChatMessage,
    crate::tui::chat::message::MessageType,
);
