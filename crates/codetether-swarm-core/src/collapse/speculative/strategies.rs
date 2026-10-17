//! Ordered default strategies for speculative branches.

pub const STRATEGY_PROMPTS: &[&str] = &[
    "Plan carefully before writing any code. Analyze the full codebase context first, then implement.",
    "Write tests first, then implement to make them pass. Use TDD approach.",
    "Make the minimal possible change to accomplish the task. Prefer small diffs.",
    "Refactor surrounding code for clarity while implementing. Leave the codebase cleaner.",
];
