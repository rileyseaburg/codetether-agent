//! Native session data and timestamp effects used by title operations.
use codetether_provider_core::Message;
/// Session fields needed for generating and updating user-facing titles.
/// # Implementors
/// Supply borrowed native fields and the existing modification timestamp update.
pub trait TitleSession {
    /// Current title, including an explicitly stored empty value if present.
    fn title(&self) -> Option<&str>;
    /// Ordered native transcript.
    fn messages(&self) -> &[Message];
    /// Replace the native title field with an already-normalized value.
    fn store_title(&mut self, title: Option<String>);
    /// Update the native modification timestamp.
    fn touch(&mut self);
}
