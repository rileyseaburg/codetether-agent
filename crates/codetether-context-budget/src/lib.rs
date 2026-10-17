//! Context-budget reporting over the existing session working window.
//!
//! [ContextBudgetTool] loads the calling session through the shared context
//! loader. [BudgetSource] supplies counts and page labels without copying the
//! transcript. Reporting does not call providers or modify the session.
mod execute;
mod render;
mod source;
mod tool;
pub use source::BudgetSource;
/// Report working messages, cached summaries, pinned pages and effective length.
///
/// # Examples
/// ```rust
/// # fn example<S:codetether_context_budget::BudgetSource>() {
/// let _tool=codetether_context_budget::ContextBudgetTool::<S>::new();
/// # }
/// ```
pub struct ContextBudgetTool<S>(std::marker::PhantomData<fn() -> S>);
impl<S: BudgetSource> ContextBudgetTool<S> {
    /// Construct a reporter using the existing session storage contract.
    pub const fn new() -> Self {
        Self(std::marker::PhantomData)
    }
}
impl<S: BudgetSource> Default for ContextBudgetTool<S> {
    fn default() -> Self {
        Self::new()
    }
}
