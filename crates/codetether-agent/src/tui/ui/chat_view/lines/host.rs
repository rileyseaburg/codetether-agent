//! Native renderers and cache access for the shared chat-line pipeline.
mod agent;
mod transcript;
use crate::tui::{
    app::state::App, color_palette::ColorPalette, message_formatter::MessageFormatter,
};
use codetether_message_cache::{DrawnLines, RenderInputs, RenderedLineCache};
use ratatui::text::Line;
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub formatter: &'a MessageFormatter,
    pub palette: &'a ColorPalette,
}
impl codetether_message_cache::pipeline::Host for Native<'_> {
    fn swarm_lines(&mut self, content_width: usize) -> Option<DrawnLines> {
        transcript::selected(self, content_width)
    }
    fn agent_lines(&mut self, content_width: usize) -> Option<DrawnLines> {
        agent::build(self, content_width)
    }
    fn cache_inputs(&self) -> RenderInputs {
        RenderInputs {
            pending_tool: self.app.state.tool_timing.pending_tool_started_at.is_some(),
            messages_len: self.app.state.messages.len(),
            tool_preview_scroll: self.app.state.tool_viewport.tool_preview_scroll,
            processing: self.app.state.processing,
        }
    }
    fn cache(&mut self) -> &mut RenderedLineCache {
        &mut self.app.state.message_cache
    }
    fn build_uncached(&mut self, separator_width: usize, panel_width: usize) -> Vec<Line<'static>> {
        transcript::main(self, separator_width, panel_width)
    }
    fn push_streaming(&self, lines: &mut Vec<Line<'static>>, separator_width: usize) {
        super::super::streaming::push_streaming_preview(
            lines,
            &self.app.state,
            separator_width,
            self.formatter,
        );
    }
}
