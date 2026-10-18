//! Management view render bindings.
use ratatui::Frame;
/// Application render bindings for management views.
/// # Implementors
/// Native adapters pass each view's state to its renderer.
pub trait Management {
    /// Render the file picker view.
    fn file_picker(&mut self, frame: &mut Frame);
    /// Render the audit view.
    fn audit(&mut self, frame: &mut Frame);
    /// Render the git view.
    fn git(&mut self, frame: &mut Frame);
    /// Render the audit loop view.
    fn audit_loop(&mut self, frame: &mut Frame);
    /// Render the editor view.
    fn editor(&mut self, frame: &mut Frame);
}
