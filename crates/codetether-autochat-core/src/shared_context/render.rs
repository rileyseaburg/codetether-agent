//! Shared relay memory render operations.
use super::*;
impl SharedRelayContext {
    pub fn render_prompt_snapshot(&self) -> String {
        if self.is_empty() {
            return String::new();
        }

        let mut lines = Vec::new();
        push_section(&mut lines, "Facts", &self.facts);
        push_section(&mut lines, "Decisions", &self.decisions);
        push_section(&mut lines, "Risks", &self.risks);
        push_section(&mut lines, "Next Actions", &self.next_actions);
        push_section(&mut lines, "Evidence", &self.evidence);
        lines.join("\n")
    }
}
