//! Workspace session-list refresh and status handling.
//!
//! Implement [`Source`] and [`View`] for native listing and UI state, then call
//! [`refresh`] to replace the picker contents or report the existing failure.
mod host;
mod service;
pub use host::{Source, View};
pub use service::refresh;
