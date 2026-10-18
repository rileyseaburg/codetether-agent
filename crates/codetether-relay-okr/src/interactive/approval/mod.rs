//! Pending relay-objective approval, modal keys and persistence.
//!
//! [handle] routes approval keys. [handle_okr_approve] records a decision,
//! schedules persistence and starts the relay; [handle_okr_deny] clears the
//! pending proposal and reports denial. [Host] supplies frontend operations.
mod actions;
mod host;
mod keys;
mod save;
pub use actions::{handle_okr_approve, handle_okr_deny};
pub use host::Host;
pub use keys::{handle, is_action};
