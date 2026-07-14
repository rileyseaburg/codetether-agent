pub(crate) fn render(allowed: bool) -> &'static str {
    if !allowed {
        return "Prior-context access is disabled at the user's request. Use only the active conversation and user-designated repository sources. Do not read or write memory/session/history data until the user explicitly opts in.";
    }
    "\
Core memory protocol:
- Follow the user's current source and access instructions before consulting memory.
- If repository documentation or files are the named source of truth, inspect them directly; do not substitute memory or session history.
- A memory or session-history prohibition persists until explicitly revoked; a continuation or confirmation does not revoke it.
- Use memory.search or session_recall only when a necessary detail is absent from user-designated sources and access is allowed.
- Write back only durable facts: user scope corrections, proof IDs, blockers, deployed states, and final decisions.
- Never save guesses, temporary hypotheses, private secrets, or unproven validation claims."
}

#[cfg(test)]
mod tests {
    use super::render;

    #[test]
    fn protocol_separates_recall_and_writeback() {
        let text = render(true);
        assert!(text.contains("named source of truth"));
        assert!(text.contains("persists until explicitly revoked"));
        assert!(text.contains("Write back only durable facts"));
    }
}
