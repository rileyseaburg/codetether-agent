//! Answer-satisfaction popup content and explicit yes/no selection.
//!
//! Use [render] only when the host's durable review is ready. [choose] reports
//! user intent; [interactive::handle] records that intent through host storage
//! bindings before applying UI feedback.
mod content;
mod input;
pub mod interactive;
mod render;
pub use content::lines;
pub use input::choose;
pub use render::render;
