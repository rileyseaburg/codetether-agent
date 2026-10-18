//! Compact session, tool and shortcut inspector for the webview layout.
//!
//! Supply a borrowed [View] to [build] for lines or [render] for a bordered panel.
mod lines;
mod render;
mod view;
pub use lines::build;
pub use render::render;
pub use view::View;
