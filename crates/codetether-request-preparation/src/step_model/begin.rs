use super::{Host, StepVars};
use codetether_rate_gate::check_rate_gate;
use std::sync::Arc;
/// Restore the original target and optionally substitute a rate-gated model.
/// # Arguments
/// `vars` borrows active fields; `host` supplies native services;
/// `rate_gate` enables proactive substitution for interactive sessions.
/// # Returns
/// Success after model restoration and proactive context preparation.
/// # Errors
/// Propagates missing-provider failures from restoration or substitution.
/// # Examples
/// ```no_run
/// # fn begin<R, S>(vars: codetether_request_preparation::step_model::StepVars<'_, R, S>, host: &impl codetether_request_preparation::step_model::Host<R, S>) -> anyhow::Result<()> {
/// codetether_request_preparation::step_model::begin_step(vars, host, true)?;
/// # Ok(()) }
/// ```
pub fn begin_step<R, S>(
    mut vars: StepVars<'_, R, S>,
    host: &impl Host<R, S>,
    rate_gate: bool,
) -> anyhow::Result<()> {
    super::restore_step_model(&mut vars, host)?;
    if !rate_gate {
        return done(vars, host);
    }
    let Some((sub_p, sub_m)) = check_rate_gate(vars.selected_provider, vars.model) else {
        return done(vars, host);
    };
    tracing::info!(substitute_provider = %sub_p, substitute_model = %sub_m,
        "Rate gate: using substitute for this step");
    super::apply_failover(vars, sub_p, sub_m, host)
}
fn done<R, S>(vars: StepVars<'_, R, S>, host: &impl Host<R, S>) -> anyhow::Result<()> {
    host.prepare(vars.session, Arc::clone(vars.provider), vars.model);
    Ok(())
}
