//! Ralph run overview and selected-story detail rendering.
//!
//! [crate::RalphViewState] supplies the run and selection. Call
//! [render_ralph_view] to draw its overview or detail panel into a frame.
mod detail_error;
mod detail_header;
mod detail_hints;
mod detail_merge;
mod detail_messages;
mod detail_output;
mod detail_tools;
mod header_style;
mod item_passed;
mod item_quality;
mod item_running;
mod message_entry;
mod render;
mod render_footer;
mod render_header;
mod render_progress;
mod render_story_detail;
mod render_story_list;
mod run_counts;
mod run_identity;
mod story_dependencies;
mod story_identity;
mod story_item;
mod story_quality;
mod story_status;
mod tool_entry;
mod tool_previews;
mod truncate;
pub use render::render_ralph_view;

mod detail_content;
