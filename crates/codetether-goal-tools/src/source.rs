//! Evidence supplied by the runtime rather than model-controlled tool arguments.
/// Link from a proposed goal to its canonical user message.
///
/// # Examples
/// ```
/// let source = codetether_goal_tools::GoalSource {
///     turn_id: "message:12".into(), text_hash: "sha256-digest".into(), verbatim: false,
/// };
/// assert!(!source.verbatim);
/// ```
pub struct GoalSource {
    /// Absolute canonical message index, prefixed with `message:`.
    pub turn_id: String,
    /// SHA-256 of the original message's joined text parts, before trimming.
    pub text_hash: String,
    /// Whether the proposed objective equals the source text after trimming.
    pub verbatim: bool,
}
