use super::Snapshot;
/// Native update operations for one periodic UI tick.
/// # Implementors
/// TUI adapters retain concrete subsystem state and event queues.
pub trait Host {
    /// Refresh question-wait state before taking the baseline snapshot.
    fn question_wait(&mut self);
    /// Capture visible change counters and current animation epoch.
    fn snapshot(&self) -> Snapshot;
    /// Drain autochat messages.
    fn autochat(&mut self);
    /// Drain completed voice transcriptions.
    fn voice(&mut self);
    /// Drain Ralph updates and report visible changes.
    fn ralph(&mut self) -> bool;
    /// Drain swarm updates and report visible changes.
    fn swarm(&mut self) -> bool;
    /// Drain forage updates and report visible changes.
    fn forage(&mut self) -> bool;
    /// Drain shell updates and report visible changes.
    fn shell(&mut self) -> bool;
    /// Drain loaded history pages and report visible changes.
    fn history(&mut self) -> bool;
    /// Drain symbol refreshes and report visible changes.
    fn symbols(&mut self) -> bool;
    /// Reconcile approvals and report visible changes.
    fn approvals(&mut self) -> bool;
    /// OR the tick's redraw decision into the existing redraw flag.
    fn mark_redraw(&mut self, changed: bool);
}
