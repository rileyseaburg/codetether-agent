//! Selection between managed-child and tool-child detail rows.
use ratatui::{style::Stylize, text::Line};
/// Selected-child lookup and existing detail sources.
/// # Implementors
/// Native adapters supply the active name, managed sessions and tool registry.
pub trait Host {
    /// Currently selected child name.
    fn selected(&self) -> Option<&str>;
    /// Managed-child rows when this name belongs to a managed session.
    fn managed(&self, name: &str) -> Option<Vec<Line<'static>>>;
    /// Tool-child or peer rows for the selected name.
    fn tool(&self, name: &str) -> Vec<Line<'static>>;
}
/// Choose detail rows for the focused child.
/// # Arguments
/// * `host` - Selected name and both detail sources.
/// # Returns
/// Managed details first, otherwise tool details; a hint when none is selected.
/// # Examples
/// ```no_run
/// fn detail(host: &impl codetether_agent_dashboard::detail::Host) {
///     let rows = codetether_agent_dashboard::detail::lines(host);
///     assert!(!rows.is_empty());
/// }
/// ```
pub fn lines(host: &impl Host) -> Vec<Line<'static>> {
    let Some(name) = host.selected() else {
        return vec![Line::from("No managed agent selected.".dim())];
    };
    if let Some(rows) = host.managed(name) {
        return rows;
    }
    host.tool(name)
}
