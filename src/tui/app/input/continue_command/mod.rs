//! Frontend continuation and session-picker resource selection.
codetether_chat_submit::declare_frontend_continuation!(
    pub(in crate::tui::app::input);
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::provider::ProviderRegistry,
    crate::tui::worker_bridge::TuiWorkerBridge,
    crate::tui::app::session_runtime::TuiSessionHandle,
    crate::session::Session,
    crate::provider::Message,
    codetether_chat_model::ChatMessage,
    crate::tui::app::watchdog::WatchdogNotification;
    crate::session::tasks::runtime::resume_prompt,
    crate::tui::app::input::chat_helpers::push_user_messages,
    crate::tui::app::input::chat_submit_dispatch::dispatch_prompt;
    codetether_session_picker::interaction::Host,
    codetether_session_picker::SessionPickerState,
    crate::tui::app::codex_sessions::load_selected_session,
    codetether_session_picker::interaction::select,
);
