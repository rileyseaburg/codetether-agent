//! Parsed auto-chat request parameters.

/// Parsed participant count, task and command flags.
///
/// # Examples
/// ```
/// use codetether_autochat_core::ParsedAutochatRequest;
/// let request = ParsedAutochatRequest {
///     agent_count: 3, task: "review".into(), bypass_prd: false, explicit_count: false,
/// };
/// assert_eq!(request.task, "review");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedAutochatRequest {
    pub agent_count: usize,
    pub task: String,
    pub bypass_prd: bool,
    pub explicit_count: bool,
}
