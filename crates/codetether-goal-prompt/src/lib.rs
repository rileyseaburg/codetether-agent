//! Goal-setup prompt editing, response delivery, keyboard input and rendering.
//!
//! Create [GoalPromptState] with a response channel and pass its optional state
//! to [handle_key] and [render_if_active]. Use [handle_key_with_feedback] to
//! apply [KeyOutcome] to the application's status and redraw state.
mod area;
mod edit;
mod feedback;
mod finish;
mod footer;
mod goals;
mod header;
mod input;
mod key;
mod outcome;
mod render;
mod state;
pub use feedback::handle_key_with_feedback;
pub use key::handle_key;
pub use outcome::KeyOutcome;
pub use render::{render, render_if_active};
pub use state::GoalPromptState;
#[cfg(test)]
mod tests;
