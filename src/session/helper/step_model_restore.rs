//! Native specialization of shared step-model restoration and failover.
#[path = "step_model_host.rs"]
mod host;
#[path = "step_prepare.rs"]
pub(super) mod step_prepare;
use crate::{provider::ProviderRegistry, session::Session, tool::ToolRegistry};
pub(super) type StepVars<'a> =
    codetether_request_preparation::step_model::StepVars<'a, ToolRegistry, Session>;
pub(super) fn restore_step_model(
    vars: &mut StepVars<'_>,
    registry: &ProviderRegistry,
) -> anyhow::Result<()> {
    codetether_request_preparation::step_model::restore_step_model(vars, &host::Native(registry))
}
pub(super) fn apply_failover(
    vars: StepVars<'_>,
    retry_provider: String,
    retry_model: String,
    registry: &ProviderRegistry,
) -> anyhow::Result<()> {
    codetether_request_preparation::step_model::apply_failover(
        vars,
        retry_provider,
        retry_model,
        &host::Native(registry),
    )
}
