//! Adapter-specific history conventions.
/// Existing conventions for ordinary text and packed tool results.
///
/// # Examples
/// ```
/// use codetether_provider_zai::history::HistoryFormat;
/// let format = HistoryFormat::SingleReply;
/// assert_eq!(match format {
///     HistoryFormat::SingleReply => 1,
///     HistoryFormat::ExpandedReplies => 2,
/// }, 1);
/// ```
#[derive(Clone, Copy)]
pub enum HistoryFormat {
    /// Application: newline text, first content part as tool reply.
    SingleReply,
    /// Injected client: concatenate text and emit every tool reply.
    ExpandedReplies,
}
