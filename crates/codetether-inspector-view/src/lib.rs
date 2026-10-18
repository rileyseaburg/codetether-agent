//! Completion metrics and recent-tool inspector panels.
//!
//! Supply an [InspectorView] and call [render_inspector] to fill a frame,
//! or render [render_metrics] and [render_tool_history] in custom layouts.
mod footer;
mod layout;
mod metric_line;
mod metrics;
mod row;
mod tool_display;
mod tool_history;
mod tool_lines;
mod view;
pub use layout::render_inspector;
pub use metrics::render_metrics;
pub use tool_history::render_tool_history;
pub use view::InspectorView;
