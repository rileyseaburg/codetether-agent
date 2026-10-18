//! Frontend message and missing-provider effects for shared publication.
codetether_chat_submit::declare_frontend_publication!(
    crate::tui::app::state::App,
    crate::session::ImageAttachment,
    crate::tui::worker_bridge::TuiWorkerBridge,
    codetether_chat_model::ChatMessage,
    crate::tui::app::worker_bridge::handle_processing_stopped,
);
