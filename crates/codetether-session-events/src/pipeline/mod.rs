//! Ordered frontend delivery of session runtime events.
//!
//! [handle] records activity, runs the [Stages] in order and trims retained
//! frontend state whether or not a stage consumes the event. [View] supplies
//! activity and final-presentation state without depending on a UI type.
mod run;
mod stages;
mod view;
pub use run::handle;
pub use stages::Stages;
pub use view::View;

mod binding;
