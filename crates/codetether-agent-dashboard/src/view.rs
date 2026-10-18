//! Selection between dashboard and child-detail panels.
use ratatui::{Frame, layout::Rect, text::Line};
/// Read-only panel inputs and row builders.
/// # Implementors
/// Native view adapters supply selection and delegate to shared row composition.
pub trait Host {
    /// Whether the child detail panel is active.
    fn detail_mode(&self) -> bool;
    /// Focused child's name, when available.
    fn selected(&self) -> Option<&str>;
    /// Current vertical detail offset.
    fn scroll(&self) -> u16;
    /// Compose the overview rows.
    fn dashboard_lines(&self) -> Vec<Line<'static>>;
    /// Compose the focused child's detail rows.
    fn detail_lines(&self) -> Vec<Line<'static>>;
}
/// Render either the dashboard or selected child's details.
/// # Arguments
/// * `f` - Destination frame.
/// * `area` - Available panel rectangle.
/// * `host` - Panel selection and row builders.
/// # Returns
/// Draws the active agent panel.
/// # Examples
/// ```no_run
/// fn draw(f: &mut ratatui::Frame, host: &impl codetether_agent_dashboard::view::Host) {
///     let area = f.area();
///     codetether_agent_dashboard::view::render(f, area, host);
/// }
/// ```
pub fn render(f: &mut Frame, area: Rect, host: &impl Host) {
    if host.detail_mode() {
        crate::render::detail(f, area, host.selected(), host.scroll(), host.detail_lines());
        return;
    }
    crate::render::dashboard(f, area, host.dashboard_lines());
}
