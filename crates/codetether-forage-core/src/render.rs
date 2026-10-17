//! Text rendering for forage run summaries.
use crate::ForageRunSummary;
impl ForageRunSummary {
    /// Render cycle totals, up to five selected items, and execution totals.
    pub fn render_text(&self) -> String {
        let mut lines = vec![format!(
            "Forage completed {} cycle(s); selected {} opportunity(s) total.",
            self.cycles_completed, self.total_selected
        )];

        if self.last_cycle_selected.is_empty() {
            lines.push("No forage opportunities were selected in the final cycle.".to_string());
        } else {
            lines.push(format!(
                "Final cycle selected {} opportunity(s):",
                self.last_cycle_selected.len()
            ));
            for (idx, item) in self.last_cycle_selected.iter().take(5).enumerate() {
                lines.push(format!(
                    "{}. {} -> {} (score {:.3}, remaining {:.1}%)",
                    idx + 1,
                    item.okr_title,
                    item.key_result_title,
                    item.score,
                    item.remaining * 100.0
                ));
            }
            if self.last_cycle_selected.len() > 5 {
                lines.push(format!(
                    "... and {} more final-cycle opportunities",
                    self.last_cycle_selected.len() - 5
                ));
            }
        }

        if self.execute_requested {
            lines.push(format!(
                "Execution engine: {}. Attempted {} execution(s) with {} failure(s).",
                self.execution_engine, self.total_executed, self.total_execution_failures
            ));
        }

        lines.join("\n")
    }
}
