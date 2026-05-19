//! Many-worlds speculative development — race N branches, keep the winner.

use super::collapse_controller::CollapsePolicy;
use anyhow::Result;

pub const STRATEGY_PROMPTS: &[&str] = &[
    "Plan carefully before writing any code. Analyze the full codebase context first, then implement.",
    "Write tests first, then implement to make them pass. Use TDD approach.",
    "Make the minimal possible change to accomplish the task. Prefer small diffs.",
    "Refactor surrounding code for clarity while implementing. Leave the codebase cleaner.",
];

pub struct SpeculativeRunner {
    pub branch_count: usize,
    pub strategies: Vec<String>,
    pub policy: CollapsePolicy,
}

impl SpeculativeRunner {
    pub fn new(branch_count: usize, strategies: Vec<String>) -> Self {
        Self {
            branch_count: branch_count.clamp(1, 8),
            strategies,
            policy: CollapsePolicy::default(),
        }
    }

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

pub struct BranchSpec {
    pub branch_name: String,
    pub strategy_prompt: String,
    pub task_prompt: String,
}
