//! Autochat relay integration for the TUI event loop.

pub mod events;
pub mod handler;
pub use codetether_autochat_core::interactive::notify;
pub use codetether_autochat_core::interactive::persona;
pub use codetether_autochat_core::interactive::persona_pick;
pub use codetether_autochat_core::interactive::persona_state;
pub use codetether_autochat_core::interactive::request;
pub mod run;
pub use codetether_autochat_core::interactive::run_step;
pub mod state;
pub use codetether_autochat_core::interactive::step_request;
pub use codetether_autochat_core::interactive::summary;
pub mod worker;

pub use handler::handle_autochat_event;

pub(crate) mod host;
