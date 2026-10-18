//! Frontend state and callbacks for shared paste handling.
codetether_paste_buffer::declare_frontend_paste!(
    crate::tui::app::state::App,
    crate::tui::models::InputMode,
    crate::tui::models::ViewMode;
    crate::tui::app::input::image_data_paste::try_attach_data_url,
    crate::tui::app::symbols::symbol_search_active,
    crate::tui::app::symbols::schedule_refresh,
);
