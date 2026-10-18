//! Bounded text and item retention for live terminal views.
//!
//! [bounded] truncates text with the existing diagnostic marker and
//! [push_capped] retains newest items. [agent] and [bus] apply the established
//! view-specific limits; [agent_update] updates optional output slots.
pub mod agent;
pub mod agent_update;
mod bounded;
pub mod bus;
mod items;
mod limits;
pub use bounded::bounded;
pub use items::push_capped;
pub use limits::*;

/// Bounded chat history retention with viewport preservation.
pub mod chat;

mod application_binding;
