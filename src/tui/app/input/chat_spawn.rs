//! Frontend type selection for owned-session provider handoff.
codetether_chat_submit::declare_frontend_handoff!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::provider::ProviderRegistry,
    crate::tui::app::session_runtime::TuiSessionHandle,
    crate::session::Session,
    crate::tui::app::session_runtime::PromptRequest,
    crate::session::ImageAttachment,
);
