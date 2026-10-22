//! Pin or unpin a calling-session turn and persist its page classification.
//!
//! [ContextPinTool] validates the action and index, loads the calling session,
//! then updates and saves through [PinSource]. Page mutation remains delegated
//! to the existing tracked storage container.
mod correct;
mod execute;
pub use execute::apply_to_session;
mod logic;
mod schema;
mod source;
mod supersede;
#[cfg(test)]
mod tests;
mod tool;
pub use source::PinSource;
/// Context pinning tool bound to the application's session storage.
///
/// # Examples
/// ```rust
/// # fn example<S:codetether_context_pin::PinSource>() {
/// let _tool=codetether_context_pin::ContextPinTool::<S>::new();
/// # }
/// ```
pub struct ContextPinTool<S>(std::marker::PhantomData<fn() -> S>);
impl<S: PinSource> ContextPinTool<S> {
    /// Create a stateless pinning tool for the selected session type.
    pub const fn new() -> Self {
        Self(std::marker::PhantomData)
    }
}
impl<S: PinSource> Default for ContextPinTool<S> {
    fn default() -> Self {
        Self::new()
    }
}
