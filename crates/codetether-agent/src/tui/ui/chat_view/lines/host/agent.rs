//! Bind the current agent selection to shared detail lookup and rendering.
use super::Native;
use codetether_message_cache::DrawnLines;
use codetether_tool_panel::TranscriptViewport;

pub(super) fn build(host: &mut Native<'_>, width: usize) -> Option<DrawnLines> {
    let state = &mut host.app.state;
    let rows = codetether_agent_dashboard::selection::known_lines(
        state.agent_tree.active.as_deref(),
        &state.agent_tree.spawned,
        state.session_id.as_deref(),
        &codetether_agent_dashboard::tool_detail::Registry::<
            crate::tool::agent::bridge::ApplicationHost,
        >::new(),
    );
    TranscriptViewport {
        viewport: &mut state.tool_viewport,
        formatter: host.formatter,
        palette: host.palette,
    }
    .detail(rows, width)
}
