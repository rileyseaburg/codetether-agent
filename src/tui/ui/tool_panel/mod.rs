//! Tool-activity panel: groups, renders, and decorates inline tool calls.

pub mod arg_preview;
mod arg_preview_helpers;
mod entry_builder;
mod format_meta;
mod formatted;
pub mod icons;
mod image_file;
mod item;
mod item_call;
mod item_result;
mod item_thinking;
mod panel;
mod panel_chrome;
mod pending_spinner;
mod preview;
mod preview_excerpt;
mod render_chat;

pub use entry_builder::{build_render_entries, separator_pattern};
pub use panel::{PendingToolSnapshot, build_tool_activity_panel};
pub use render_chat::render_chat_message;

use crate::tui::chat::message::{ChatMessage, MessageType};

/// Max visible lines inside the compact tool panel.
pub const TOOL_PANEL_VISIBLE_LINES: usize = 6;

#[derive(Default)]
pub struct RenderEntry<'a> {
    pub tool_activity: Vec<&'a ChatMessage>,
    pub message: Option<&'a ChatMessage>,
}

pub struct ToolPanelRender {
    pub lines: Vec<ratatui::text::Line<'static>>,
    pub max_scroll: usize,
}

pub fn is_tool_activity(message_type: &MessageType) -> bool {
    matches!(
        message_type,
        MessageType::ToolCall { .. } | MessageType::ToolResult { .. } | MessageType::Thinking(_)
    )
}
