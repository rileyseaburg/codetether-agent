//! Rendering operations and cache fields required by the line pipeline.
use crate::{DrawnLines, RenderInputs, RenderedLineCache};
use ratatui::text::Line;
/// Native transcript renderers sharing cache assembly policy.
/// # Implementors
/// Supply focused-agent rendering, main transcript construction and live suffixes.
pub trait Host {
    /// Render a focused swarm transcript when one is active.
    fn swarm_lines(&mut self, content_width: usize) -> Option<DrawnLines>;
    /// Render a focused agent transcript when one is active.
    fn agent_lines(&mut self, content_width: usize) -> Option<DrawnLines>;
    /// Project current cache validation inputs.
    fn cache_inputs(&self) -> RenderInputs;
    /// Borrow the owned full-frame cache.
    fn cache(&mut self) -> &mut RenderedLineCache;
    /// Rebuild main-chat lines and update any native scroll bounds.
    fn build_uncached(&mut self, separator_width: usize, panel_width: usize) -> Vec<Line<'static>>;
    /// Append current streaming content after the frozen prefix.
    fn push_streaming(&self, lines: &mut Vec<Line<'static>>, separator_width: usize);
}
