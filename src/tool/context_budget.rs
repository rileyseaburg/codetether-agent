//! Bind context budget reporting to the application's existing session data.
use crate::session::{Session, pages::PageKind};
/// Context reporter using the application's session store.
///
/// # Examples
/// ```rust
/// use codetether_agent::tool::context_budget::ContextBudgetTool;
/// let _tool: ContextBudgetTool = ContextBudgetTool;
/// ```
pub type ContextBudgetTool = codetether_context_budget::ContextBudgetTool<Session>;
/// Preserve the existing unit-like construction used by tool registration.
///
/// # Examples
/// ```rust
/// let _tool = codetether_agent::tool::context_budget::ContextBudgetTool;
/// ```
#[allow(non_upper_case_globals)]
pub const ContextBudgetTool: ContextBudgetTool =
    codetether_context_budget::ContextBudgetTool::new();
impl codetether_context_budget::BudgetSource for Session {
    const WINDOW: usize = crate::session::store::WINDOW;
    fn messages_len(&self) -> usize {
        self.messages.len()
    }
    fn cached_summaries(&self) -> usize {
        self.summary_index.len()
    }
    fn pages(&self) -> &[PageKind] {
        self.pages.as_slice()
    }
}
