//! Autochat relay integration for the TUI event loop.

codetether_autochat_core::declare_frontend_autochat!(
    crate::tui::app::state::AppState,
    crate::session::SessionEvent,
    crate::provider::ProviderRegistry,
    crate::provider::Provider,
    crate::tui::chat::message::ChatMessage,
    crate::tui::chat::message::MessageType,
);
pub use codetether_autochat_core::interactive::notify;
pub use codetether_autochat_core::interactive::persona;
pub use codetether_autochat_core::interactive::persona_pick;
pub use codetether_autochat_core::interactive::persona_state;
pub use codetether_autochat_core::interactive::request;
pub use codetether_autochat_core::interactive::run_step;
pub use codetether_autochat_core::interactive::step_request;
pub use codetether_autochat_core::interactive::summary;

pub use handler::handle_autochat_event;
