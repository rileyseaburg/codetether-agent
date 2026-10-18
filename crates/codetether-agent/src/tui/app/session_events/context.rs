//! Frontend types for shared context telemetry presentation.
codetether_context_health::declare_context_events!(
    crate::tui::app::state::App,
    crate::session::SessionEvent,
    crate::tui::chat::message::ChatMessage,
    crate::tui::chat::message::MessageType,
);
