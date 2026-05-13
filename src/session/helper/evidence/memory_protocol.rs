pub(crate) fn render() -> &'static str {
    "\
Core memory protocol:
- Recall first when a task references previous scope, proof, blockers, or decisions.
- Let high-confidence beliefs bias which memories and session history you search first.
- Prefer memory.search with tags core-memory/evidence/scope before asking the user to repeat.
- Write back only durable facts: user scope corrections, proof IDs, blockers, deployed states, and final decisions.
- Never save guesses, temporary hypotheses, private secrets, or unproven validation claims."
}

#[cfg(test)]
mod tests {
    use super::render;

    #[test]
    fn protocol_separates_recall_and_writeback() {
        let text = render();
        assert!(text.contains("Recall first"));
        assert!(text.contains("Write back only durable facts"));
    }
}
