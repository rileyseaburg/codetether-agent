use super::{ContextTrace, ContextTraceSummary};
use std::collections::HashMap;

impl ContextTrace {
    /// Get a summary of the trace.
    pub fn summary(&self) -> ContextTraceSummary {
        let mut event_counts = HashMap::new();
        let mut event_tokens = HashMap::new();
        self.count_events(&mut event_counts, &mut event_tokens);
        ContextTraceSummary {
            total_tokens: self.total_tokens,
            max_tokens: self.max_tokens,
            budget_used_percent: self.budget_used_percent(),
            iteration: self.iteration,
            event_counts,
            event_tokens,
            events_len: self.events.len(),
        }
    }

    fn count_events(
        &self,
        event_counts: &mut HashMap<String, usize>,
        event_tokens: &mut HashMap<String, usize>,
    ) {
        for event in &self.events {
            let label = event.label().to_string();
            *event_counts.entry(label.clone()).or_insert(0) += 1;
            *event_tokens.entry(label).or_insert(0) += event.tokens();
        }
    }
}
