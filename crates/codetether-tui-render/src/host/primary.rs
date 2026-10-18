//! Primary view render bindings.
use ratatui::Frame;
/// Application render bindings for primary views.
/// # Implementors
/// Native adapters pass each view's state to its renderer.
pub trait Primary {
    /// Render the sessions view.
    fn sessions(&mut self, frame: &mut Frame);
    /// Render the swarm view.
    fn swarm(&mut self, frame: &mut Frame);
    /// Render the ralph view.
    fn ralph(&mut self, frame: &mut Frame);
    /// Render the bus view.
    fn bus(&mut self, frame: &mut Frame);
    /// Render the subagents view.
    fn subagents(&mut self, frame: &mut Frame);
    /// Render the model view.
    fn model(&mut self, frame: &mut Frame);
    /// Render the settings view.
    fn settings(&mut self, frame: &mut Frame);
}
