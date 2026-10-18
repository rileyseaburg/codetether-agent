//! Borrowed history fields added to a session header for full JSON snapshots.
use crate::HeaderView;
use codetether_agent_core::ToolUse;
use codetether_context_policy::{pages::PageKind, summary_index::SummaryIndex};
use codetether_provider_core::Message;
/// Snapshot input that never clones the transcript graph.
/// # Examples
/// ```rust
/// # fn example<'a>(header: codetether_session_codec::HeaderView<'a>, index: &'a codetether_context_policy::summary_index::SummaryIndex) {
/// let snapshot = codetether_session_codec::SnapshotView {
///     header, messages: &[], pages: &[], summary_index: index, tool_uses: &[],
/// };
/// assert!(snapshot.messages.is_empty());
/// # }
/// ```
pub struct SnapshotView<'a> {
    /// Native identity and metadata fields.
    pub header: HeaderView<'a>,
    /// Ordered transcript messages.
    pub messages: &'a [Message],
    /// Existing page sidecar; regenerated only if lengths disagree.
    pub pages: &'a [PageKind],
    /// Hierarchical summary cache.
    pub summary_index: &'a SummaryIndex,
    /// Ordered tool-use records.
    pub tool_uses: &'a [ToolUse],
}
