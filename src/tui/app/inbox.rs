//! Frontend selection for interactive peer admission and dispatch.
codetether_a2a_interactive::declare_frontend_inbox!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::provider::ProviderRegistry,
    crate::tui::worker_bridge::TuiWorkerBridge,
    crate::tui::app::session_runtime::TuiSessionHandle,
    crate::tui::worker_bridge::IncomingTask;
    crate::tui::chat::message::ChatMessage,
    crate::tui::chat::message::MessageType,
    codetether_a2a_inbox::Inbound;
    crate::tui::app::input::chat_submit_dispatch::dispatch_prompt,
);
