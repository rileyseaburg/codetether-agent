//! Steering queue methods.
//!
//! Steering messages are queued guidance injected into the next LLM turn.

impl super::AppState {
    pub fn queue_steering(&mut self, value: impl Into<String>) {
        let trimmed = value.into().trim().to_string();
        if !trimmed.is_empty() {
            self.queued_steering.push(trimmed);
        }
    }

    pub fn clear_steering(&mut self) {
        self.queued_steering.clear();
    }

    pub fn steering_count(&self) -> usize {
        self.queued_steering.len()
    }

    pub fn steering_prompt_prefix(&self) -> Option<String> {
        if self.queued_steering.is_empty() {
            return None;
        }
        let items = self
            .queued_steering
            .iter()
            .enumerate()
            .map(|(idx, item)| format!("{}. {item}", idx + 1))
            .collect::<Vec<_>>()
            .join("\n");
        Some(format!(
            "[Queued steering for this turn]\nApply the following guidance while answering:\n{items}\n"
        ))
    }
}
