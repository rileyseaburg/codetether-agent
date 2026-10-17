use super::*;
impl SpeculativeRunner {
    /// Construct ordered branch plans with configured or fallback strategies.
    ///
    /// # Arguments
    /// * `_base_path` - Retained for API compatibility; planning does not use it.
    /// * `task_prompt` - Task text copied into each branch plan.
    ///
    /// # Returns
    /// One specification per configured branch; this function performs no I/O.
    ///
    /// # Errors
    /// The current implementation does not produce an error.
    ///
    /// # Examples
    /// ```
    /// let runner = codetether_swarm_core::collapse::speculative::SpeculativeRunner::new(2, vec![]);
    /// let plans = runner.build_branches(std::path::Path::new("."), "inspect").unwrap();
    /// assert_eq!(plans.len(), 2);
    /// ```
    pub fn build_branches(
        &self,
        _base_path: &std::path::Path,
        task_prompt: &str,
    ) -> Result<Vec<BranchSpec>> {
        let mut specs = Vec::with_capacity(self.branch_count);
        for i in 0..self.branch_count {
            let fallback = STRATEGY_PROMPTS[i % STRATEGY_PROMPTS.len()];
            let strategy = self
                .strategies
                .get(i)
                .map(String::as_str)
                .unwrap_or(fallback);
            specs.push(BranchSpec {
                branch_name: format!("speculative-{}", i),
                strategy_prompt: strategy.to_string(),
                task_prompt: task_prompt.to_string(),
            });
        }
        Ok(specs)
    }
}
