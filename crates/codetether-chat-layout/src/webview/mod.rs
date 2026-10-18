//! Webview chat mode and responsive panel geometry.
//!
//! [compute] lays out header, composer, suggestions and status. [webview_body_chunks]
//! arranges the sidebar and inspector; [show_sidebar] and [show_inspector] keep
//! narrow terminals readable. [ChatLayoutMode] retains the user's layout choice.
mod body;
mod main;
mod mode;
mod status;
pub use body::{is_webview, show_inspector, show_sidebar, webview_body_chunks};
pub use main::{MainChunks, compute};
pub use mode::ChatLayoutMode;
pub use status::status_height;

mod sidebar;
pub use sidebar::sidebar_chunks;
