/// Produce a lightweight view of an owned session.
/// # Implementors
/// Frontends select their view type and copy only the metadata they display.
pub trait ViewSource {
    /// View available while the full session is checked out for execution.
    type View;
    /// Capture the current view without transferring session ownership.
    fn snapshot(&self) -> Self::View;
}
