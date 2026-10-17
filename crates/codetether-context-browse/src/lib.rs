//! Browse the calling session as bounded, materialized transcript turn files.
//!
//! [ContextBrowseTool] validates actions and loads session metadata. [BrowseSource]
//! provides bounded durable reads; shared history helpers render and name files.
//! [ContextBrowseAction] and [parse_browse_action] expose the input contract.
mod action;
mod execute;
mod fault;
mod indexed;
mod listing;
mod parse;
mod schema;
mod source;
mod tool;
pub use action::ContextBrowseAction;
pub use codetether_history_files::format_turn_path;
pub use parse::parse_browse_action;
pub use source::BrowseSource;
/// Bounded history browser bound to a durable session source.
///
/// # Examples
/// ```rust
/// # fn example<S:codetether_context_browse::BrowseSource>() {
/// let _tool=codetether_context_browse::ContextBrowseTool::<S>::new();
/// # }
/// ```
pub struct ContextBrowseTool<S>(std::marker::PhantomData<fn() -> S>);
impl<S: BrowseSource> ContextBrowseTool<S> {
    /// Construct a stateless browser using the specified session source.
    pub const fn new() -> Self {
        Self(std::marker::PhantomData)
    }
}
impl<S: BrowseSource> Default for ContextBrowseTool<S> {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests;
