//! Answer-satisfaction popup content and explicit yes/no selection.
//!
//! Use [render] only when the host's durable review is ready. [choose] reports
//! user intent; the host remains responsible for recording and applying it.
mod content;
mod input;
mod render;
pub use content::lines;
pub use input::choose;
pub use render::render;
