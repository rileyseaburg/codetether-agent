//! Overflow recovery that prioritizes prepared context over compaction.

use crate::derivation::DerivePolicy;

const TIGHT_PREPARED_BUDGET: usize = 8_192;

/// One bounded retry action after a context-window rejection.
///
/// # Examples
/// ```
/// use codetether_context_policy::request::recovery::Plan;
/// let action = Plan::Emergency { keep_last: 3 };
/// assert!(matches!(action, Plan::Emergency { keep_last: 3 }));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Plan {
    /// Retry using a tighter prepared-context budget.
    Prepared { budget_tokens: usize },
    /// Retry using forced keep-last compaction.
    Emergency { keep_last: usize },
}

pub(super) fn for_attempt(attempt: usize, policy: DerivePolicy) -> Option<Plan> {
    if attempt == 1 {
        if let DerivePolicy::Incremental { budget_tokens } = policy {
            let budget = if budget_tokens == 0 {
                TIGHT_PREPARED_BUDGET
            } else {
                (budget_tokens / 2).max(1)
            };
            return Some(Plan::Prepared {
                budget_tokens: budget,
            });
        }
    }
    let offset = usize::from(matches!(policy, DerivePolicy::Incremental { .. }));
    super::retry_plan::keep_last_for_attempt(attempt.saturating_sub(offset))
        .map(|keep_last| Plan::Emergency { keep_last })
}
