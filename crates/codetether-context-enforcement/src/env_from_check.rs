//! Capture scalar limits and trace identity for a cascade.
use super::enforce_attempt::CascadeEnv;

impl<'a> CascadeEnv<'a> {
    pub(super) fn from_check(
        trace_id: uuid::Uuid,
        trigger_reason: &'a str,
        check: &super::enforce_budget::BudgetCheck,
    ) -> Self {
        Self {
            trace_id,
            trigger_reason,
            initial_est: check.initial_est,
            safety_budget: check.safety_budget,
            ctx_window: check.ctx_window,
        }
    }
}
