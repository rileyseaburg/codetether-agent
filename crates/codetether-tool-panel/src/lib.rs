//! Tool activity grouping, previews, diffs and terminal rendering.
//!
//! [build_render_entries] groups tool activity for [build_tool_activity_panel].
//! [render_chat_message] formats ordinary chat messages. The host supplies a
//! lazy approval-tool lookup when constructing a panel with pending work.
include!("modules.rs");
pub use activity::is_tool_activity;
pub use entry_builder::{build_render_entries, separator_pattern};
pub use panel::{PendingToolSnapshot, build_tool_activity_panel};
pub use render_chat::render_chat_message;
pub use types::{RenderEntry, ToolPanelRender};
/// Max visible rows in the compact activity panel.
pub const TOOL_PANEL_VISIBLE_LINES: usize = 12;
/// Scroll sentinel for following the latest tool activity.
pub const TOOL_PREVIEW_FOLLOW: usize = 1_000_000;

mod entries;
mod entry_result;
mod separator;
pub use entries::append_entries;
pub use entry_result::EntryAppendResult;
pub use separator::push_separator;

mod entry_panel;

mod streaming_preview;
pub use streaming_preview::push_streaming_preview;

mod transcript;
pub use transcript::build_transcript;

mod scroll_bounds;
mod scroll_navigation;
mod scroll_state;
pub use scroll_state::ToolPreviewScrollState;

/// Structured tool-error parsing and preview formatting.
pub mod error;
/// Bounded tool-argument and output-preview settings.
pub mod limits;

/// Application-local UI state adapters.
pub mod state_binding;

#[cfg(test)]
mod scroll_tests;
