//! Bind the header, sidebar, inspector and cached transcript renderers.
use super::Native;
use ratatui::{Frame, layout::Rect};
impl codetether_webview::Panels for Native<'_> {
    fn header(&mut self, f: &mut Frame, area: Rect) {
        super::super::header::render_webview_header(f, self.0, area);
    }
    fn sidebar(&mut self, f: &mut Frame, area: Rect) {
        super::super::sidebar::render_webview_sidebar(f, self.0, area);
    }
    fn inspector(&mut self, f: &mut Frame, area: Rect) {
        super::super::inspector::render_webview_inspector(f, self.0, area);
    }
    fn center(&mut self, f: &mut Frame, area: Rect) {
        super::super::chat_lines::render_center(f, self.0, area);
    }
}
