//! Native renderers and cache access for the shared chat-line pipeline.
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
        super::super::swarm_lines::build(self.app, content_width, self.formatter, self.palette)
    }
    fn agent_lines(&mut self, content_width: usize) -> Option<DrawnLines> {
        super::super::agent_lines::build(self.app, content_width, self.formatter)
    }
    fn cache_inputs(&self) -> RenderInputs {
        self.app.state.render_cache_inputs()
    }
    fn cache(&mut self) -> &mut RenderedLineCache {
        &mut self.app.state.message_cache
    }
    fn build_uncached(&mut self, separator_width: usize, panel_width: usize) -> Vec<Line<'static>> {
        super::super::build_uncached::build_uncached(
            self.app,
            separator_width,
            panel_width,
            self.formatter,
            self.palette,
        )
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
