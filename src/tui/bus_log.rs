//! Protocol bus log view for the TUI.
//!
//! The bus log module turns [`crate::bus::BusEnvelope`] values into
//! render-ready rows, keeps scroll/filter state, and draws the protocol view.

mod entry;
mod entry_agent;
mod entry_agent_lifecycle;
mod entry_agent_message;
mod entry_parts;
mod entry_ralph;
mod entry_ralph_parts;
mod entry_router;
mod entry_speech;
mod entry_task;
mod entry_tool;
mod entry_tool_output;
mod entry_tool_request;
mod entry_tool_thinking;
mod entry_voice;
mod entry_voice_transcript;
mod render;
mod render_body;
mod render_detail;
mod render_footer;
mod render_item;
mod render_list;
mod render_summary;
mod render_summary_data;
mod render_summary_lines;
mod render_summary_status;
mod render_summary_text;
mod state;
mod state_detail;
mod state_filter;
mod state_nav;
mod summary;
mod truncate;

#[cfg(test)]
mod state_filter_tests;
#[cfg(test)]
mod state_mode_tests;
#[cfg(test)]
mod test_support;

pub use entry::BusLogEntry;
pub use render::{render_bus_log, render_bus_log_with_summary};
pub use state::BusLogState;
pub use summary::ProtocolSummary;
