//! Frontend type and callback selection for view-level Enter routing.
codetether_tui_navigation::declare_frontend_enter!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::provider::ProviderRegistry,
    crate::tui::worker_bridge::TuiWorkerBridge,
    crate::tui::app::session_runtime::TuiSessionHandle,
    crate::tui::models::ViewMode;
    crate::tui::app::file_picker::file_picker_enter,
    crate::tui::app::input::bus::handle_enter_bus_filter,
    crate::tui::app::input::model_apply::selected,
    crate::tui::app::input::enter_subagents::dispatch;
    crate::tui::app::input::sessions::handle_enter_sessions,
    crate::tui::app::settings::toggle_selected_setting,
    crate::tui::app::input::chat_submit::handle_enter_chat,
);
