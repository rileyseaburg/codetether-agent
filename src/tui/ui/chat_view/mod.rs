//! # Chat View Sub-modules
//!
//! Decomposed into single-responsibility modules. Start with
//! [`render_chat_view`] for the full pipeline.

pub mod agent_bar;
pub mod agent_bar_tool;
pub mod agent_bar_tui;
pub mod agent_rail;
pub mod agent_tab;
pub(crate) mod approval_overlay;
pub mod attachment;
pub mod auto_apply;
pub mod badges;
mod build_uncached;
pub mod compact_hints;
pub mod context_gauge;
pub mod cursor;
pub mod diff_lines;
mod drawn_lines;
pub mod elapsed_badge;
pub mod empty;
pub mod entries;
pub mod entry_result;
pub mod format_cache;
pub mod hit;
pub mod images_badge;
pub mod input_area;
pub mod layout_chunks;
pub mod layout_compute;
pub mod lines;
pub mod messages;
pub mod processing_badge;
pub mod render;
pub mod scroll;
pub mod scroll_indicator;
pub mod scrollbar_render;
pub mod separator;
pub mod spinner;
pub mod status;
pub mod status_hints;
pub mod status_line;
pub mod status_metrics;
pub mod status_pack;
pub mod status_text;
pub mod streaming;
pub mod streaming_header;
pub mod suggestions;
pub mod throughput_sparkline;
pub mod title;
pub mod token_spans;
pub mod turn_badge;
pub mod yolo_badge;

pub use render::render_chat_view;
