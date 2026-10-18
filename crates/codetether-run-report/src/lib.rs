//! Final CLI output for session, relay and Ralph execution.
//!
//! Call [render_session], [render_relay] or [render_ralph] after execution.
//! Each preserves the run command's JSON, JSONL and default text presentation.
mod ralph;
mod relay;
mod session;
pub use ralph::render_ralph;
pub use relay::render_relay;
pub use session::render_session;
