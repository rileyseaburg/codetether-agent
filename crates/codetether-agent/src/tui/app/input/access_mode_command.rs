//! Frontend session and callback selection for access-mode commands.
codetether_settings_policy::declare_frontend_access_command!(
    crate::tui::app::state::App,
    crate::session::Session,
    crate::provider::ProviderRegistry,
    crate::config::Config,
    crate::tui::chat::message::ChatMessage,
    crate::tui::chat::message::MessageType;
    crate::tui::app::model_picker::verifier_command,
    crate::tui::app::input::approval_command::release_all,
);
