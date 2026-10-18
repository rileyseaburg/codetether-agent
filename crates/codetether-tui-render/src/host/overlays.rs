//! Global overlay render bindings.
use ratatui::Frame;
/// Global overlays layered over the selected view.
/// # Implementors
/// Native adapters bind each overlay to its application state.
pub trait Overlays {
    /// Whether symbol search is active.
    fn symbols_active(&self) -> bool;
    /// Render the symbols overlay.
    fn symbols(&mut self, frame: &mut Frame);
    /// Render the goal prompt overlay.
    fn goal_prompt(&mut self, frame: &mut Frame);
    /// Render the fuzzy find overlay.
    fn fuzzy_find(&mut self, frame: &mut Frame);
    /// Render the interlude overlay.
    fn interlude(&mut self, frame: &mut Frame);
    /// Render the watchdog overlay.
    fn watchdog(&mut self, frame: &mut Frame);
    /// Render the answer review overlay.
    fn answer_review(&mut self, frame: &mut Frame);
    /// Render the question overlay.
    fn question(&mut self, frame: &mut Frame);
}
