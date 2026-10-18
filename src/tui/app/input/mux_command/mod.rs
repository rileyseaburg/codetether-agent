//! Frontend type selection for mux command presentation.
codetether_mux_command::declare_frontend_mux_command!(
    crate::tui::app::state::App,
    crate::tui::chat::message::ChatMessage,
    crate::tui::chat::message::MessageType,
);
