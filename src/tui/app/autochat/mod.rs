//! Autochat relay integration for the TUI event loop.

pub mod events;
pub mod handler;
pub mod notify;
pub mod persona;
pub mod persona_pick;
pub mod persona_state;
pub mod request;
pub mod run;
pub mod run_step;
pub mod state;
pub mod step_request;
pub mod summary;
pub mod worker;

pub use handler::handle_autochat_event;
