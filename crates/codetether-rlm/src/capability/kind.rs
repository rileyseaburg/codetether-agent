//! Allowed treatment of a tool output.

/// What RLM is allowed to do with a tool's output.
///
/// # Examples
/// ```
/// use codetether_rlm::{capability::{OutputCapability,output_capability}};
/// assert_eq!(output_capability("read"),OutputCapability::ExactContent);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputCapability {
    /// Bulk web/aggregate output where a prose summary is an
    /// acceptable lossy projection.
    BulkSummarizable,
    /// Content-carrying output whose exact bytes may be needed later.
    ExactContent,
    /// Unknown tool. Treated as [`OutputCapability::ExactContent`].
    Unknown,
}
