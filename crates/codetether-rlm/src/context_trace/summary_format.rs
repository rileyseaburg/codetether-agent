use super::ContextTraceSummary;

impl ContextTraceSummary {
    /// Format as a human-readable string.
    pub fn format(&self) -> String {
        let mut lines = self.header_lines();
        self.push_event_lines(&mut lines);
        lines.join("\n")
    }

    fn header_lines(&self) -> Vec<String> {
        vec![
            format!("Context Trace Summary (iteration {})", self.iteration),
            format!(
                "  Budget: {}/{} tokens ({:.1}%)",
                self.total_tokens, self.max_tokens, self.budget_used_percent
            ),
            format!("  Events: {}", self.events_len),
        ]
    }

    fn push_event_lines(&self, lines: &mut Vec<String>) {
        if self.event_counts.is_empty() {
            return;
        }
        lines.push("  By type:".to_string());
        for (label, count) in &self.event_counts {
            let tokens = self.event_tokens.get(label).copied().unwrap_or(0);
            lines.push(format!("    {label}: {count} events, {tokens} tokens"));
        }
    }
}
