//! Diagnostics view render bindings.
use ratatui::Frame;
/// Application render bindings for diagnostics views.
/// # Implementors
/// Native adapters pass each view's state to its renderer.
pub trait Diagnostics {
    /// Render the lsp view.
    fn lsp(&mut self, frame: &mut Frame);
    /// Render the rlm view.
    fn rlm(&mut self, frame: &mut Frame);
    /// Render the latency view.
    fn latency(&mut self, frame: &mut Frame);
    /// Render the transport view.
    fn transport(&mut self, frame: &mut Frame);
    /// Render the protocol view.
    fn protocol(&mut self, frame: &mut Frame);
    /// Render the inspector view.
    fn inspector(&mut self, frame: &mut Frame);
}
