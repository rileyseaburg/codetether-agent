//! Page content readers for the native backend.

mod html;
mod snapshot;
mod text;
mod title;

/// HTML reader.
pub(super) use html::html;
/// Snapshot reader.
pub(super) use snapshot::snapshot;
/// Text reader.
pub(super) use text::text;
/// Title reader.
pub(super) use title::title;
