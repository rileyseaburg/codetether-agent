//! Cost-budget enforcement for the agentic loop.
//!
//! Checked before every provider request. Reads [`CostGuardrails`] from
//! env vars (`CODETETHER_COST_WARN_USD`, `CODETETHER_COST_LIMIT_USD`)
//! and compares against the running session cost estimate from
//! [`crate::provider::pricing::session_cost_usd`].

use std::sync::atomic::{AtomicBool, Ordering};

use crate::config::guardrails::CostGuardrails;
use crate::provider::pricing::session_cost_usd;

/// One-shot latch so we don't spam the log every loop iteration once the
/// warn threshold is crossed.
static WARNED: AtomicBool = AtomicBool::new(false);

/// Result of a budget check.
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

fn check() -> CostGuardStatus {
    let g = CostGuardrails::from_env();
    let spent = session_cost_usd();

    if let Some(limit) = g.hard_limit_usd
        && spent >= limit
    {
        return CostGuardStatus::Block {
            spent_usd: spent,
            limit_usd: limit,
        };
    }
    if let Some(warn) = g.warn_usd
        && spent >= warn
        && !WARNED.swap(true, Ordering::Relaxed)
    {
        return CostGuardStatus::Warned {
            spent_usd: spent,
            threshold_usd: warn,
        };
    }
    CostGuardStatus::Ok
}

/// Enforce the cost budget before sending a provider request.
///
/// Returns `Err` if the hard limit has been reached. Logs a one-shot
/// warning the first time the warn threshold is crossed. Returns `Ok(())`
/// in the no-limits-configured case.
pub fn enforce_cost_budget() -> anyhow::Result<()> {
    match check() {
        CostGuardStatus::Block {
            spent_usd,
            limit_usd,
        } => {
            anyhow::bail!(
                "Cost guardrail tripped: session has spent ~${:.2} which meets/exceeds the \
                 hard limit of ${:.2}. Raise CODETETHER_COST_LIMIT_USD (or \
                 `[guardrails] hard_limit_usd` in config) to continue.",
                spent_usd,
                limit_usd
            )
        }
        CostGuardStatus::Warned {
            spent_usd,
            threshold_usd,
        } => {
            tracing::warn!(
                spent_usd,
                threshold_usd,
                "Cost guardrail warn threshold reached; set CODETETHER_COST_LIMIT_USD to cap spend"
            );
            Ok(())
        }
        CostGuardStatus::Ok => Ok(()),
    }
}

/// Non-mutating probe used by UI widgets to color the cost badge.
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

/// UI-facing severity of the current session's spend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CostGuardLevel {
    Ok,
    OverWarn,
    OverLimit,
}
