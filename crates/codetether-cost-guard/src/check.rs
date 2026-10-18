//! Hard-limit-first budget checks with one process-wide warning latch.
use crate::CostGuardStatus;
use codetether_config::guardrails::CostGuardrails;
use codetether_provider_metrics::session_cost_usd;
use std::sync::atomic::{AtomicBool, Ordering};

/// One-shot latch so we don't spam the log every loop iteration once the
/// warn threshold is crossed.
static WARNED: AtomicBool = AtomicBool::new(false);

pub(super) fn check() -> CostGuardStatus {
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
