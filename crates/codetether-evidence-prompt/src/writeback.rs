//! Render durable memory writeback guidance subject to permission.
/// Render durable memory writeback guidance subject to permission.
/// # Arguments
/// `allowed` indicates whether prior-context access is permitted.
/// # Returns
/// The established instruction text, with its original labels and ordering.
/// # Examples
/// ```
/// let text = codetether_evidence_prompt::writeback::render(false);
/// assert!(!text.is_empty());
/// ```
pub fn render(allowed: bool) -> &'static str {
    if !allowed {
        return "Memory writeback hook: disabled at the user's request.";
    }
    "\
Memory writeback hook:
- Save durable user scope corrections after they are acknowledged.
- Save proof IDs, artifact paths, blockers, and deployed states after validation.
- Update beliefs only when evidence succeeds, fails, or becomes stale."
}
