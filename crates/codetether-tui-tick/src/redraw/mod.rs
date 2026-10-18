//! Streaming redraw rate limiting with immediate keyboard response.
//!
//! [draw_if_ready] delegates rendering through [Host] and clears dirty state
//! only after a rendered frame. [ready] preserves the 66ms streaming cadence;
//! [finish] records a completed draw without altering skipped frames.
mod policy;
mod run;
pub use policy::{finish, ready};
pub use run::draw_if_ready;
use std::time::Instant;
/// Native terminal rendering and current redraw state.
/// # Implementors
/// UI adapters supply the real draw operation and scalar input state.
pub trait Host {
    /// Error returned by native rendering.
    type Error;
    /// Borrow the flag indicating that a new draw is needed.
    fn needs_redraw(&mut self) -> &mut bool;
    /// Whether a request is streaming.
    fn processing(&self) -> bool;
    /// Most recent keyboard event time.
    fn last_input(&self) -> Option<Instant>;
    /// Render once, reporting whether a frame was actually drawn.
    /// # Errors
    /// Propagates native rendering failures without recording completion.
    fn draw(&mut self) -> Result<bool, Self::Error>;
}
