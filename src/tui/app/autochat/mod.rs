//! Autochat relay integration for the TUI event loop.

pub mod events;
pub mod handler;
pub mod notify;
pub mod request;
pub mod run;
pub mod state;
pub mod summary;
pub mod worker;

pub use handler::handle_autochat_event;
