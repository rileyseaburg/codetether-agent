pub(crate) fn render() -> &'static str {
    "\
Memory writeback hook:
- Save durable user scope corrections after they are acknowledged.
- Save proof IDs, artifact paths, blockers, and deployed states after validation.
- Update beliefs only when evidence succeeds, fails, or becomes stale."
}
