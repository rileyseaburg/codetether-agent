//! Frontend type selection for shared scheduled model retries.
codetether_model_failover::declare_frontend_retry!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::provider::ProviderRegistry,
    crate::tui::app::session_runtime::TuiSessionHandle,
    crate::tui::app::session_runtime::PromptRequest,
);
