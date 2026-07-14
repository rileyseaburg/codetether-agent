pub(crate) fn render(allowed: bool) -> &'static str {
    if !allowed {
        return "Memory trapdoor: disabled at the user's request; do not inspect or save prior-context data.";
    }
    "\
Memory trapdoor:
- Assume prior chat details may be absent from active context.
- Do not stuff the transcript into every request.
- Preserve a compact scope/evidence ledger in your answer.
- Save durable decisions, blockers, proof IDs, and user scope corrections to core memory.
- When details are missing, honor the user's source and access instructions; use context_browse, session_recall, or memory only when prior-context access is allowed."
}

#[cfg(test)]
mod tests {
    use super::render;

    #[test]
    fn trapdoor_names_recall_tools() {
        let text = render(true);
        assert!(text.contains("context_browse"));
        assert!(text.contains("session_recall"));
        assert!(text.contains("only when prior-context access is allowed"));
    }
}
