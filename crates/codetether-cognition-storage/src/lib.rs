//! Application cognition-state storage and persistence diagnostics.
//!
//! [load_state] and [save_state] use the shared cognition snapshot format and
//! the configured application data directory. Path resolution, load/save entry
//! points and diagnostics are separate modules. Shared cognition core retains
//! capture order, size/schema guards, atomic writes and recovery behavior.
mod diagnostics;
mod load;
mod path;
mod save;
pub use codetether_cognition_core::persistence::PersistedCognitionState;
use diagnostics::{log_load, log_saved};
pub use load::load_state;
pub use save::save_state;
