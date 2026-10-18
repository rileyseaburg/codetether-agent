//! Borrow application transcript inputs for the crate-owned renderer.
use super::Native;
use codetether_message_cache::DrawnLines;
use codetether_tool_panel::TranscriptViewport;
use ratatui::text::Line;

fn approval_tool() -> Option<String> {
    crate::tui::app::state::approval_queue::active().map(|item| item.tool)
}

pub(super) fn selected(host: &mut Native<'_>, width: usize) -> Option<DrawnLines> {
    let messages = super::super::super::swarm_messages::active(&host.app.state);
    TranscriptViewport {
        viewport: &mut host.app.state.tool_viewport,
        formatter: host.formatter,
        palette: host.palette,
    }
    .selected(messages, width, approval_tool)
}

pub(super) fn main(host: &mut Native<'_>, separator: usize, panel: usize) -> Vec<Line<'static>> {
    let state = &mut host.app.state;
    TranscriptViewport {
        viewport: &mut state.tool_viewport,
        formatter: host.formatter,
        palette: host.palette,
    }
    .build(
        &state.messages,
        separator,
        panel,
        state.tool_timing.pending_tool_snapshot(),
        approval_tool,
    )
}
