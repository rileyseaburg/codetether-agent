//! Optional primary-session step budget.

/// Reports whether an explicitly configured budget is exhausted.
pub(super) fn exhausted(step: usize, max_steps: Option<usize>) -> bool {
    max_steps.is_some_and(|limit| step >= limit)
}

#[cfg(test)]
#[path = "budget_tests.rs"]
mod tests;
