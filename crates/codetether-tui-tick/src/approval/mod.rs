//! Reflect externally resolved approvals in the visible overlay state.
//!
//! [reconcile] removes stale display records through [Host] and updates the
//! waiting flag, preview scroll and status only when entries were removed.
mod run;
pub use run::reconcile;
/// Mutable display fields updated after external approval resolution.
pub struct State<'a> {
    /// Whether at least one pending approval remains.
    pub waiting: &'a mut bool,
    /// Preview scroll reset when the active queue changes.
    pub preview_scroll: &'a mut u16,
    /// Visible reconciliation status.
    pub status: &'a mut String,
}
/// Native approval queue and display-state bindings.
/// # Implementors
/// UI adapters retain the queue's existing decision and stale-record policy.
pub trait Host {
    /// Remove stale display records and return how many were removed.
    fn remove_stale(&mut self) -> usize;
    /// Read the remaining queue length after removal.
    fn remaining(&self) -> usize;
    /// Borrow fields belonging to the approval display.
    fn state(&mut self) -> State<'_>;
}
