//! Interactive session loading, continuation and archive-import workflows.
//!
//! Implement [Host] for storage and [View] for presentation. Call
//! [load_selected_session] to restore a selected session or
//! [import_workspace_sessions] to refresh the workspace archive catalog.
mod host;
mod import;
mod selection;
pub use host::{Host, Session, View};
pub use import::import_workspace_sessions;
pub use selection::load_selected_session;
