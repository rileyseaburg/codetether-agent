pub(crate) fn render() -> &'static str {
    "\
Memory trapdoor:
- Assume prior chat details may be absent from active context.
- Do not stuff the transcript into every request.
- Preserve a compact scope/evidence ledger in your answer.
- Save durable decisions, blockers, proof IDs, and user scope corrections to core memory.
- When details are missing, call context_browse, context_budget, session_recall, memory, or TetherScript validators."
}

#[cfg(test)]
mod tests {
    use super::render;

    #[test]
    fn trapdoor_names_recall_tools() {
        let text = render();
        assert!(text.contains("context_browse"));
        assert!(text.contains("session_recall"));
    }
}
