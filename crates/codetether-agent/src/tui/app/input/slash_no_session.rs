//! Frontend callbacks for commands that require no mutable session.
codetether_slash_command::declare_frontend_session_free!(
    crate::tui::app::state::App,
    crate::provider::ProviderRegistry,
    crate::tui::models::ViewMode,
    codetether_image::interactive::Presentation;
    crate::tui::app::file_share::command,
    crate::tui::app::model_picker::open_without_session,
    crate::tui::app::input::attach_image_command,
);
