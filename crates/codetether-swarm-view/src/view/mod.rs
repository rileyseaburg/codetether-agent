//! Swarm overview, subtask list and selected-agent detail rendering.
//!
//! [crate::SwarmViewState] supplies execution and selection data.
//! [render_swarm_view] draws the overview or detail into a terminal frame.
mod detail_content;
mod detail_error;
mod detail_header;
mod detail_hints;
mod detail_messages;
mod detail_output;
mod detail_tools;
mod header_line;
mod header_title;
mod message_entry;
mod render;
mod render_agent_detail;
mod render_header;
mod render_stage_progress;
mod render_stats;
mod render_subtask_list;
mod stats_line;
pub mod swarm_view_fmt;
mod swarm_view_list;
mod swarm_view_row;
mod task_dependencies;
mod task_identity;
mod task_status;
mod tool_entry;
mod tool_previews;
mod truncate;
pub use render::render_swarm_view;

mod row_activity;
