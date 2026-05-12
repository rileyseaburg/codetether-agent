use super::ContextTrace;

impl ContextTrace {
    /// Increment the iteration counter.
    pub fn next_iteration(&mut self) {
        self.iteration += 1;
    }

    /// Get the current iteration number.
    pub fn iteration(&self) -> usize {
        self.iteration
    }

    /// Get the total tokens used.
    pub fn total_tokens(&self) -> usize {
        self.total_tokens
    }

    /// Get the remaining token budget.
    pub fn remaining_tokens(&self) -> usize {
        self.max_tokens.saturating_sub(self.total_tokens)
    }

    /// Get the percentage of budget used.
    pub fn budget_used_percent(&self) -> f32 {
        if self.max_tokens == 0 {
            return 0.0;
        }
        let total = self.total_tokens as u64;
        let max = self.max_tokens as u64;
        let tenths = (total.saturating_mul(1000) + (max / 2)).saturating_div(max);
        (tenths as f32) / 10.0
    }

    /// Check if the budget is exceeded.
    pub fn is_over_budget(&self) -> bool {
        self.total_tokens > self.max_tokens
    }
}
