//! Concrete application factory identities for provider-backed tools.
/// Provider-backed tool requested from the application factory.
///
/// # Examples
/// ```
/// use codetether_tool_construction::provider::ProviderTool;
/// let kind = ProviderTool::Ralph;
/// assert!(matches!(kind, ProviderTool::Ralph));
/// ```
#[derive(Debug, Clone, Copy)]
pub enum ProviderTool {
    /// Context summarization using the application's session producer.
    ContextSummarize,
    /// Recursive language model processing with the application's router.
    Rlm,
    /// Session recall using the application's indexed history.
    SessionRecall,
    /// Autonomous Ralph execution consuming the supplied provider.
    Ralph,
}
