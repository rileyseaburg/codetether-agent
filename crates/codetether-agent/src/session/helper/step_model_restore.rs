//! Native specialization of shared model preparation.
#[path = "step_model_host.rs"]
mod host;
#[path = "step_prepare.rs"]
pub(super) mod step_prepare;
use crate::{provider::ProviderRegistry, session::Session, tool::ToolRegistry};
pub(super) type StepVars<'a> =
    codetether_request_preparation::step_model::StepVars<'a, ToolRegistry, Session>;
pub(crate) fn begin_step(
    vars: StepVars<'_>,
    registry: &ProviderRegistry,
    rate_gate: bool,
) -> anyhow::Result<()> {
    codetether_request_preparation::step_model::begin_step(vars, &host::Native(registry), rate_gate)
}

pub(super) fn resolve_initial(
    session: &Session,
    registry: &ProviderRegistry,
) -> anyhow::Result<codetether_request_preparation::model::ModelState<ToolRegistry>> {
    codetether_request_preparation::model::resolve(session, &host::Native(registry))
}
