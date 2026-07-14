//! Per-step begin hook: restore the original model, then apply proactive
//! rate-gate failover when the primary is near its RPM/TPM budget.

use anyhow::Result;

use super::step_model_restore::StepVars;
use crate::provider::ProviderRegistry;
use crate::session::rate_gate::check_rate_gate;

/// Restore the session's original model, then (optionally) swap to a rate-gate
/// substitute for this step when the primary model is near its budget.
pub(crate) fn begin_step(
    mut vars: StepVars<'_>,
    registry: &ProviderRegistry,
    rate_gate: bool,
) -> Result<()> {
    super::step_model_restore::restore_step_model(&mut vars, registry)?;
    if !rate_gate {
        return super::step_model_restore::step_prepare::done(
            vars.session,
            vars.provider,
            vars.model,
        );
    }
    let Some((sub_p, sub_m)) = check_rate_gate(vars.selected_provider, vars.model) else {
        return super::step_model_restore::step_prepare::done(
            vars.session,
            vars.provider,
            vars.model,
        );
    };
    tracing::info!(substitute_provider = %sub_p, substitute_model = %sub_m,
        "Rate gate: using substitute for this step");
    super::step_model_restore::apply_failover(vars, sub_p, sub_m, registry)
}
