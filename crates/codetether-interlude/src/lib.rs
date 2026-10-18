//! Tether Catch play-break state, scoring, input, and terminal rendering.
//!
//! Create [InterludeState], apply [handle_key], and call [render] while active.
//! [interactive] handles activation and redraw through borrowed frontend fields.
mod area;
mod board;
mod clock;
mod input;
mod movement;
mod render;
mod rules;
mod state;
pub use input::handle_key;
pub use render::render;
pub use state::InterludeState;
/// Number of horizontal lanes in the play area.
pub const LANES: u8 = 5;
/// Number of rows a falling star traverses.
pub const ROWS: u8 = 7;

/// Play-break activation and input redraw policy.
pub mod interactive;

mod application_binding;
