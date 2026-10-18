//! Terminal question overlays and session-scoped input dispatch.
//!
//! [render_pending] draws the active runtime question. [handle_key] and [paste]
//! feed user input to that same runtime; callbacks preserve host redraw and
//! activity-clock ownership. Draft validation remains in codetether-question.
mod choices;
mod help;
mod input;
mod paste;
mod render;
pub use input::handle_key;
pub use paste::paste;
pub use render::render_pending;
