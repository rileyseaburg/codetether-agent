//! Native service specialization of shared prompt-runner state.
use crate::cognition::tool_router::ToolCallRouter;
use crate::provider::ProviderRegistry;
use crate::session::Session;
use codetether_prompt_loop::RunnerState;

type NativeState<'a> = RunnerState<
    'a,
    Session,
    ProviderRegistry,
    super::model::ModelState,
    ToolCallRouter,
    super::super::prompt_events::subagent_watch::SubAgentWatch,
>;
/// Local wrapper permitting native execution-hook implementations.
pub(crate) struct Runner<'a> {
    pub inner: NativeState<'a>,
}
pub(crate) use codetether_prompt_loop::StepFlow;
