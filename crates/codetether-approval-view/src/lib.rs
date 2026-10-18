//! Approval popup content, geometry, scrolling and rendering.
//!
//! render projects the shared pending queue into a terminal popup. content_lines
//! composes full request metadata, invocation, diagnostics and reviewer findings.
mod content;
mod footer;
mod layout;
mod lsp;
mod preview;
mod render;
mod review;
mod scroll;
mod text;
pub use content::lines as content_lines;
pub use render::render;

/// Preview keyboard navigation and bounded scrolling.
pub mod input;

/// Pending-approval shortcuts and editor/copy actions.
pub mod interactive;

/// Incoming approval presentation and background-check dispatch.
pub mod request;
