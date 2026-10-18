//! Bind the worker session to shared execution and child controls.
use super::{Application, Session, SessionResult, ToolRegistry, context::Context};
use codetether_a2a_worker_core::{
    execution_policy::AutoApprove,
    session_protocol::{policy, transcript},
};
use codetether_bus_core::AgentBus;
use codetether_native_session_runtime::{RunGuard, boundary, swarm_monitor};
use std::sync::Arc;
/// Prepare and execute a controlled swarm, then continue the parent when needed.
/// # Arguments
/// Receives the canonical session, prompt, approval/model choices, task metadata,
/// optional worker guidance, bus and output observer.
/// # Returns
/// The persisted session result and its success status.
/// # Errors
/// Propagates permission, title, execution, persistence or handoff failures.
pub async fn execute<S, T, W, E>(
    session: &mut S,
    prompt: &str,
    auto_approve: AutoApprove,
    model_tier: Option<&str>,
    explicit_model: Option<String>,
    metadata: &serde_json::Map<String, serde_json::Value>,
    complexity_hint: Option<&str>,
    worker_personality: Option<&str>,
    target_agent_name: Option<&str>,
    bus: Option<&Arc<AgentBus>>,
    output_callback: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) -> anyhow::Result<(SessionResult, bool)>
where
    S: Session,
    T: Application,
    W: codetether_worker_tools::Host<Registry = ToolRegistry<T>>,
    E: codetether_swarm_runtime::inline::Executor<Bus = AgentBus>,
{
    run::<S, T, W, E>(Context {
        session,
        prompt,
        auto_approve,
        model_tier,
        explicit_model,
        metadata,
        complexity_hint,
        worker_personality,
        target_agent_name,
        bus,
        output: output_callback,
    })
    .await
}
async fn run<S, T, W, E>(mut context: Context<'_, S>) -> anyhow::Result<(SessionResult, bool)>
where
    S: Session,
    T: Application,
    W: codetether_worker_tools::Host<Registry = ToolRegistry<T>>,
    E: codetether_swarm_runtime::inline::Executor<Bus = AgentBus>,
{
    boundary::ensure_running(policy::Source::session_id(context.session)).await?;
    let mut guard = RunGuard::open(policy::Source::session_id(context.session))?;
    let control = guard.handle();
    let setup = guard.run(super::prepare::run(&mut context)).await?;
    let result = swarm_monitor::run(
        context.session,
        &control,
        codetether_swarm_runtime::inline::run::<E>(
            context.prompt,
            setup.strategy,
            setup.config,
            context.bus,
            context.output.clone(),
            control.clone(),
        ),
    )
    .await;
    transcript::Host::save(context.session).await?;
    let result = result?;
    let result = super::finish::apply(context.session, result).await?;
    guard
        .run(super::handoff::after::<S, T, W>(
            context.session,
            result,
            control.has_input(),
            context.auto_approve,
            context.model_tier,
            context.metadata,
            context.output,
        ))
        .await
}
