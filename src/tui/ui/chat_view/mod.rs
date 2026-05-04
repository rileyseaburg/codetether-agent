//! # Chat View Sub-modules
//!
//! Decomposed into single-responsibility modules. Start with
//! [`render_chat_view`] for the full pipeline.

pub mod attachment;
pub mod auto_apply;
pub mod badges;
mod build_uncached;
pub mod cursor;
pub mod elapsed_badge;
pub mod empty;
pub mod entries;
pub mod entry_result;
pub mod format_cache;
pub mod images_badge;
pub mod input_area;
pub mod layout_chunks;
pub mod layout_compute;
pub mod lines;
pub mod messages;
pub mod render;
pub mod scroll;
pub mod separator;
pub mod spinner;
pub mod status;
pub mod status_hints;
pub mod status_line;
pub mod status_text;
pub mod streaming;
pub mod suggestions;
pub mod title;
pub mod token_spans;
pub mod turn_badge;

pub use render::render_chat_view;
