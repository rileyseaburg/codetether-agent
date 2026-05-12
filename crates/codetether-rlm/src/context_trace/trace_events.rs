use super::trace::MAX_EVENTS;
use super::{ContextEvent, ContextTrace};
use std::collections::VecDeque;

impl ContextTrace {
    /// Log a context event.
    pub fn log_event(&mut self, event: ContextEvent) {
        self.total_tokens += event.tokens();
        self.evict_old_events();
        self.events.push_back(event);
    }

    fn evict_old_events(&mut self) {
        while self.events.len() >= MAX_EVENTS {
            if let Some(evicted) = self.events.pop_front() {
                self.total_tokens = self.total_tokens.saturating_sub(evicted.tokens());
            }
        }
    }

    /// Get all events.
    pub fn events(&self) -> &VecDeque<ContextEvent> {
        &self.events
    }

    /// Get events by type.
    pub fn events_of_type(&self, label: &str) -> Vec<&ContextEvent> {
        self.events
            .iter()
            .filter(|event| event.label() == label)
            .collect()
    }
}
