//! Context telemetry event handling and user-facing status updates.
//!
//! [handle_event] routes token estimates, truncations, compactions and RLM
//! progress. [View] binds the shared health state to a frontend's token counts,
//! status text and error presentation.
mod compaction;
mod context;
mod rlm;
mod view;
pub use context::handle_event;
pub use view::View;
