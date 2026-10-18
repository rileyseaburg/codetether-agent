//! Owned full-frame and frozen-prefix line-cache state.
use ratatui::text::Line;
/// Rendered chat lines with the metadata that controls reuse.
///
/// # Examples
/// ```
/// let cache = codetether_message_cache::RenderedLineCache::default();
/// assert!(cache.cached_message_lines.is_empty());
/// ```
///
/// The renderer moves lines out for one frame and restores them afterward:
///
/// ```
/// use codetether_message_cache::{RenderedLineCache, RenderInputs};
/// let mut cache = RenderedLineCache::default();
/// let inputs = RenderInputs::default();
/// cache.store_message_lines(vec![ratatui::text::Line::from("hello")], 80, inputs);
/// let lines = cache.take_cached_if_valid(80, inputs).unwrap();
/// cache.restore_cached_message_lines(lines);
/// assert_eq!(cache.cached_message_lines.len(), 1);
/// ```
#[derive(Default)]
pub struct RenderedLineCache {
    /// Fully rendered lines available for reuse.
    pub cached_message_lines: Vec<Line<'static>>,
    /// Message count when rendered.
    pub cached_messages_len: usize,
    /// Panel width when rendered.
    pub cached_max_width: usize,
    /// Whether the streaming suffix is up to date.
    pub cached_streaming_current: bool,
    /// Processing state when rendered.
    pub cached_processing: bool,
    /// Tool preview scroll when rendered.
    pub cached_tool_preview_scroll: usize,
    /// Reusable prefix length.
    pub cached_frozen_len: usize,
}
