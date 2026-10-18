//! Message-panel fields and application history hooks.
use crate::{ChatHit, ChatScrollState};
use ratatui::{style::Color, text::Line};
/// History and viewport access used before drawing the message panel.
/// # Implementors
/// Frontends bind history paging and the shared viewport state.
pub trait Scroll {
    /// Set the history paginator's visible height.
    fn set_viewport_height(&mut self, height: usize);
    /// Apply any deferred prepend anchor before clamping.
    fn apply_pending_anchor(&mut self, total_lines: usize);
    /// Borrow the chat viewport and its measured scroll bound.
    fn viewport(&mut self) -> &mut ChatScrollState;
}
/// Presentation fields required to render an interactive messages panel.
/// # Implementors
/// Frontends supply their scroll adapter, title, accent and hit recorder.
pub trait Host {
    /// History and viewport adapter.
    type Scroll: Scroll;
    /// Borrow the current history/viewport state.
    fn scroll(&mut self) -> &mut Self::Scroll;
    /// Select the accent from the current view mode.
    fn accent(&self) -> Color;
    /// Build the owned title for this frame.
    fn title(&self) -> Line<'static>;
    /// Borrow the visible-line hit recorder.
    fn hit(&mut self) -> &mut ChatHit;
}
