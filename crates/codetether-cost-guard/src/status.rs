//! Outcomes from enforcing or displaying a configured spend threshold.

/// Result of a budget check.
/// # Examples
/// ```
/// use codetether_cost_guard::CostGuardStatus;
/// let status = CostGuardStatus::Ok;
/// assert!(matches!(status, CostGuardStatus::Ok));
/// ```
#[derive(Debug, Clone, Copy)]
pub enum CostGuardStatus {
    Ok,
    /// Warn threshold crossed this call (one-shot).
    Warned {
        spent_usd: f64,
        threshold_usd: f64,
    },
    /// Hard limit crossed.
    Block {
        spent_usd: f64,
        limit_usd: f64,
    },
}

/// UI-facing severity of the current session's spend.
/// # Examples
/// ```
/// use codetether_cost_guard::CostGuardLevel;
/// let color = match CostGuardLevel::OverWarn {
///     CostGuardLevel::Ok => "normal",
///     CostGuardLevel::OverWarn => "warning",
///     CostGuardLevel::OverLimit => "error",
/// };
/// assert_eq!(color, "warning");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CostGuardLevel {
    Ok,
    OverWarn,
    OverLimit,
}
