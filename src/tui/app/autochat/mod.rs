//! Autochat relay integration for the TUI event loop.

pub mod events;
pub mod handler;
pub mod state;
pub mod worker;

pub use handler::handle_autochat_event;
