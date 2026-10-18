//! Application bindings consumed by the Webview compositor.
use ratatui::{Frame, layout::Rect};

/// Read-only layout inputs.
/// # Implementors
/// Application adapters expose composer text and suggestion visibility.
pub trait State {
    /// Current composer text used to size the input area.
    fn input(&self) -> &str;
    /// Whether the suggestions panel requires space.
    fn suggestions_visible(&self) -> bool;
}
/// Renderers for the header and body panels.
/// # Implementors
/// Application adapters delegate to their panel renderers and message cache.
pub trait Panels {
    /// Render the conversation header.
    fn header(&mut self, frame: &mut Frame, area: Rect);
    /// Render the workspace and recent-session sidebar.
    fn sidebar(&mut self, frame: &mut Frame, area: Rect);
    /// Render the compact inspector.
    fn inspector(&mut self, frame: &mut Frame, area: Rect);
    /// Render the transcript, maintaining any message cache.
    fn center(&mut self, frame: &mut Frame, area: Rect);
}
/// Renderers for composer, completions and status.
/// # Implementors
/// Application adapters bind cursor placement and status data.
pub trait Composer {
    /// Render composer text and place its cursor.
    fn input_panel(&mut self, frame: &mut Frame, area: Rect);
    /// Render completion suggestions.
    fn suggestions(&mut self, frame: &mut Frame, area: Rect);
    /// Render the status bar.
    fn status(&mut self, frame: &mut Frame, area: Rect);
}
/// Overlay renderers, called after the panels.
/// # Implementors
/// Application adapters bind approval and help state.
pub trait Overlays {
    /// Render the active approval overlay.
    fn approval(&mut self, frame: &mut Frame, area: Rect);
    /// Render help when active.
    fn help(&mut self, frame: &mut Frame);
}
/// Complete application binding used by [crate::render].
/// # Implementors
/// Any type implementing [State], [Panels], [Composer] and [Overlays].
pub trait Host: State + Panels + Composer + Overlays {}
impl<T: State + Panels + Composer + Overlays> Host for T {}
