//! Pure derivation dispatch after policy and recovery overrides.
use super::DerivePolicy;
/// Concrete derivation to execute for this request.
///
/// # Examples
/// ```
/// use codetether_context_policy::derivation::{DerivePolicy, dispatch::{Execution, select}};
/// assert_eq!(select(DerivePolicy::OracleReplay { lookahead: 3 }, None), Execution::Legacy);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Execution {
    /// Legacy derivation, including forced keep-last recovery and oracle fallback.
    Legacy,
    /// Reset with the selected threshold.
    Reset { threshold_tokens: usize },
    /// Incremental selection with the configured budget.
    Incremental { budget_tokens: usize },
}
/// Select execution without calling providers or reading session state.
///
/// # Arguments
/// * policy - Effective persisted/environment policy.
/// * force_keep_last - An explicit recovery override, including Some(0).
///
/// # Returns
/// Legacy for forced recovery or oracle replay, otherwise the selected strategy.
///
/// # Examples
/// ```
/// use codetether_context_policy::derivation::{DerivePolicy, dispatch::{Execution, select}};
/// assert_eq!(select(DerivePolicy::Reset { threshold_tokens: 100 }, Some(0)), Execution::Legacy);
/// ```
pub fn select(policy: DerivePolicy, force_keep_last: Option<usize>) -> Execution {
    if force_keep_last.is_some() {
        return Execution::Legacy;
    }
    match policy {
        DerivePolicy::Legacy | DerivePolicy::OracleReplay { .. } => Execution::Legacy,
        DerivePolicy::Reset { threshold_tokens } => Execution::Reset { threshold_tokens },
        DerivePolicy::Incremental { budget_tokens } => Execution::Incremental { budget_tokens },
    }
}
#[cfg(test)]
#[path = "dispatch_tests.rs"]
mod tests;
