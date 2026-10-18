//! Context summary tool bound to the application's producer and session store.
mod prepared;
mod source;
/// Cached or on-demand summaries using the existing application session backend.
///
/// # Examples
/// ```rust
/// let _tool=codetether_agent::tool::context_summarize::ContextSummarizeTool::cached_only();
/// ```
pub type ContextSummarizeTool =
    codetether_context_summarize::ContextSummarizeTool<crate::session::Session>;
