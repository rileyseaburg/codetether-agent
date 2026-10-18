//! User bubbles, assistant gutters and conversation timeline lines.
//!
//! [user_bubble] adds a right-aligned rounded frame around text rows.
//! [assistant_bubble_lines] preserves styled content under a fading gutter.
pub mod bubble_assistant;
pub mod timeline;
mod user;
mod user_rows;
mod user_style;
pub use bubble_assistant::assistant_bubble_lines;
pub use user::user_bubble;
