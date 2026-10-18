//! Finder-specific presentation effects over the editor view bindings.
use super::View;
/// Editor view that can request redraws after finder input.
/// # Implementors
/// Interactive frontends implement this alongside [View].
pub trait FinderView: View {
    /// Request a redraw after handling finder input.
    fn redraw(&mut self);
}
