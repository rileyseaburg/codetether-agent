// Tool-activity panel: groups, renders, and decorates inline tool calls.

mod activity;
pub mod arg_preview;
mod arg_preview_agent;
mod arg_preview_helpers;
mod diff_primitives;
mod diff_render;
mod edit_diff;
mod entry_builder;
mod format_meta;
mod formatted;
mod formatted_body;
mod formatted_header;
pub mod icons;
mod image_file;
mod item;
mod item_call;
mod item_result;
mod item_thinking;
mod line_diff;
#[cfg(test)]
mod line_diff_tests;
mod multiedit_diff;
mod panel;
mod panel_chrome;
#[cfg(test)]
mod panel_tests;
mod panel_window;
mod pending_spinner;
mod preview;
mod preview_excerpt;
mod render_chat;
mod result_preview;

#[cfg(test)]
mod peer_render_tests;

mod call_header;
mod diff_fields;
mod line_diff_flush;
mod panel_empty;
mod panel_render;
mod types;
