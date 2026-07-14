//! Overflow recovery that prioritizes prepared context over compaction.

use crate::session::derive_policy::DerivePolicy;

const TIGHT_PREPARED_BUDGET: usize = 8_192;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Plan {
    Prepared { budget_tokens: usize },
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
