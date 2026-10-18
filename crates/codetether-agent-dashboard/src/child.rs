//! Borrowed projection from an application's managed-agent type.
use crate::managed::Agent;
/// Expose only fields needed to display a managed child.
/// # Implementors
/// Application child types project identity and current status without
/// transferring ownership of their sessions or performing side effects.
pub trait Child {
    /// Borrow the child's display fields.
    fn display(&self) -> Agent<'_>;
}
