//! Cached audit-log inspection and terminal presentation.
//!
//! [AuditViewState] owns the cursor, filter and cached entries.
//! Call [refresh_audit_snapshot] on a tick, then [render_audit_view].
mod body;
mod detail;
mod detail_lines;
mod filter;
mod footer;
mod header;
mod key_value;
#[path = "list.rs"]
mod list;
mod navigation;
mod refresh;
mod render;
mod state;
pub use filter::AuditFilter;
pub use refresh::refresh_audit_snapshot;
pub use render::render_audit_view;
pub use state::AuditViewState;
