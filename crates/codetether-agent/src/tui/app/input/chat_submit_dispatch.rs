//! Frontend service selection for prompt dispatch.
codetether_chat_submit::declare_frontend_prompt_dispatch!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::provider::ProviderRegistry,
    crate::tui::worker_bridge::TuiWorkerBridge,
    crate::tui::app::session_runtime::TuiSessionHandle,
    crate::session::ImageAttachment;
    crate::session::tasks::runtime::answer_review::held,
    crate::tui::app::worker_bridge::handle_processing_started,
    crate::tui::app::input::chat_spawn::spawn_provider_task,
    crate::tui::app::input::chat_helpers::no_provider_error,
);
