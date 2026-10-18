//! Application-independent editor viewport rendering.
//!
//! Project buffer and viewport fields into [View], then use [draw_active] for
//! ordinary documents or [draw_goal] to include persistent goal-edit actions.
mod goal;
mod plain;
mod state;
pub use goal::draw_goal;
pub use plain::draw_active;
pub use state::View;
