//! Transcript rendering coupled to its tool-preview viewport.
use crate::{PendingToolSnapshot, ToolPreviewScrollState, append_entries, build_render_entries};
use codetether_chat_model::ChatMessage;
use codetether_message_cache::DrawnLines;
use codetether_message_formatter::MessageFormatter;
use codetether_theme::ColorPalette;
use ratatui::text::Line;

/// Borrows the viewport and presentation resources used to render a transcript.
///
/// # Examples
/// ```
/// let mut viewport = codetether_tool_panel::ToolPreviewScrollState::default();
/// let formatter = codetether_message_formatter::MessageFormatter::new(80);
/// let palette = codetether_theme::ColorPalette::marketing();
/// let mut renderer = codetether_tool_panel::TranscriptViewport {
///     viewport: &mut viewport, formatter: &formatter, palette: &palette,
/// };
/// assert!(renderer.selected(None, 80, || None).is_none());
/// ```
pub struct TranscriptViewport<'a> {
    /// Tool-preview scroll position and measured bounds.
    pub viewport: &'a mut ToolPreviewScrollState,
    /// Formatter used for message content.
    pub formatter: &'a MessageFormatter,
    /// Colors used for message content.
    pub palette: &'a ColorPalette,
}

impl TranscriptViewport<'_> {
    /// Render the main transcript and update its tool-preview scroll bounds.
    ///
    /// # Arguments
    /// Receives chronological messages, separator and panel widths, pending
    /// tool timing, and a lazy lookup of the current approval's tool name.
    /// # Returns
    /// Rendered lines, including the welcome banner for an empty transcript.
    /// # Examples
    /// ```
    /// # use codetether_tool_panel::{ToolPreviewScrollState, TranscriptViewport};
    /// # let mut viewport = ToolPreviewScrollState::default();
    /// # let formatter = codetether_message_formatter::MessageFormatter::new(80);
    /// # let palette = codetether_theme::ColorPalette::marketing();
    /// let mut renderer = TranscriptViewport {
    ///     viewport: &mut viewport, formatter: &formatter, palette: &palette,
    /// };
    /// assert!(!renderer.build(&[], 60, 80, None, || None).is_empty());
    /// ```
    pub fn build(
        &mut self,
        messages: &[ChatMessage],
        separator_width: usize,
        panel_width: usize,
        pending: PendingToolSnapshot<'_>,
        approval_tool: impl FnMut() -> Option<String>,
    ) -> Vec<Line<'static>> {
        let rendered = super::build_transcript(
            messages,
            separator_width,
            panel_width,
            self.viewport.tool_preview_scroll,
            pending,
            self.formatter,
            self.palette,
            approval_tool,
        );
        self.viewport
            .set_tool_preview_max_scroll(rendered.max_scroll);
        rendered.lines
    }

    /// Render a selected worker transcript without a pending-tool indicator.
    ///
    /// # Arguments
    /// Receives the selected worker's messages, available content width, and
    /// a lazy lookup of the current approval's tool name.
    /// # Returns
    /// Rebuilt lines when a worker is selected. Missing selection leaves the
    /// viewport unchanged; an empty selected transcript produces empty lines.
    /// # Examples
    /// ```
    /// # use codetether_tool_panel::{ToolPreviewScrollState, TranscriptViewport};
    /// # let mut viewport = ToolPreviewScrollState::default();
    /// # let formatter = codetether_message_formatter::MessageFormatter::new(80);
    /// # let palette = codetether_theme::ColorPalette::marketing();
    /// let mut renderer = TranscriptViewport {
    ///     viewport: &mut viewport, formatter: &formatter, palette: &palette,
    /// };
    /// let drawn = renderer.selected(Some(vec![]), 80, || None).unwrap();
    /// assert!(drawn.as_slice().is_empty());
    /// ```
    pub fn selected(
        &mut self,
        messages: Option<Vec<ChatMessage>>,
        content_width: usize,
        approval_tool: impl FnMut() -> Option<String>,
    ) -> Option<DrawnLines> {
        let messages = messages?;
        let entries = build_render_entries(&messages);
        let mut lines = Vec::new();
        let result = append_entries(
            &mut lines,
            &entries,
            content_width.saturating_sub(2).min(60),
            content_width.saturating_sub(4),
            0,
            None,
            self.formatter,
            self.palette,
            approval_tool,
        );
        self.viewport
            .set_tool_preview_max_scroll(result.tool_preview_max_scroll);
        Some(DrawnLines::from_rebuild(lines))
    }
}
