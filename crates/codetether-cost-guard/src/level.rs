//! Read-only UI severity that does not consume warning state.
use crate::CostGuardLevel;
use codetether_config::guardrails::CostGuardrails;
use codetether_provider_metrics::session_cost_usd;

/// Non-mutating probe used by UI widgets to color the cost badge.
/// # Arguments
/// Takes no arguments; reads shared thresholds and accumulated provider spend.
/// # Returns
/// Hard-limit severity first, then warning severity, otherwise Ok.
/// # Examples
/// ```
/// use codetether_cost_guard::{cost_guard_level, CostGuardLevel};
/// assert!(matches!(cost_guard_level(), CostGuardLevel::Ok | CostGuardLevel::OverWarn | CostGuardLevel::OverLimit));
/// ```
pub fn cost_guard_level() -> CostGuardLevel {
    let g = CostGuardrails::from_env();
    let spent = session_cost_usd();
    if let Some(limit) = g.hard_limit_usd
        && spent >= limit
    {
        return CostGuardLevel::OverLimit;
    }
    if let Some(warn) = g.warn_usd
        && spent >= warn
    {
        return CostGuardLevel::OverWarn;
    }
    CostGuardLevel::Ok
}
