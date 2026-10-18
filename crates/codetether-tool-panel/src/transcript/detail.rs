//! Render selected-agent detail rows inside the chat viewport.
use super::TranscriptViewport;
use codetether_message_cache::DrawnLines;
use ratatui::text::Line;

impl TranscriptViewport<'_> {
    /// Wrap selected-agent detail rows and reset tool-preview scroll bounds.
    ///
    /// # Arguments
    /// Receives resolved detail rows and the available content width.
    /// # Returns
    /// Rebuilt chat lines, or `None` without changing scroll bounds when no
    /// agent is selected or the selection is stale.
    /// # Examples
    /// ```
    /// # use codetether_tool_panel::{ToolPreviewScrollState, TranscriptViewport};
    /// # let mut viewport = ToolPreviewScrollState::default();
    /// # let formatter = codetether_message_formatter::MessageFormatter::new(80);
    /// # let palette = codetether_theme::ColorPalette::marketing();
    /// let mut renderer = TranscriptViewport {
    ///     viewport: &mut viewport, formatter: &formatter, palette: &palette,
    /// };
    /// assert!(renderer.detail(None, 80).is_none());
    /// ```
    pub fn detail(
        &mut self,
        rows: Option<Vec<Line<'static>>>,
        content_width: usize,
    ) -> Option<DrawnLines> {
        let rows = rows?;
        let width = content_width.saturating_sub(2);
        let lines = rows
            .into_iter()
            .flat_map(|line| self.formatter.wrap_line(line.spans, width))
            .collect();
        self.viewport.set_tool_preview_max_scroll(0);
        Some(DrawnLines::from_rebuild(lines))
    }
}
