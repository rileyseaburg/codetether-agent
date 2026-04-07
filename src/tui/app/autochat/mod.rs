//! Autochat relay integration for the TUI event loop.

pub mod events;
pub mod handler;
pub mod state;
pub mod worker;

pub use events::AutochatUiEvent;
pub use handler::handle_autochat_event;
pub use state::AutochatState;
pub use worker::start_autochat_relay;
