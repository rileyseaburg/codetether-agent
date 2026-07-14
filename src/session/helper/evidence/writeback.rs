pub(crate) fn render(allowed: bool) -> &'static str {
    if !allowed {
        return "Memory writeback hook: disabled at the user's request.";
    }
    "\
Memory writeback hook:
- Save durable user scope corrections after they are acknowledged.
- Save proof IDs, artifact paths, blockers, and deployed states after validation.
- Update beliefs only when evidence succeeds, fails, or becomes stale."
}
