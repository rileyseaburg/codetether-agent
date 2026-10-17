//! Bind the canonical worker session to shared execution and child controls.

use super::context::Context;
use crate::session::{
    SessionResult,
    runtime::{RunGuard, boundary, swarm_monitor},
};

pub(super) async fn run(mut context: Context<'_>) -> anyhow::Result<(SessionResult, bool)> {
    boundary::ensure_running(&context.session.id).await?;
    let mut guard = RunGuard::open(&context.session.id)?;
    let control = guard.handle();
    let setup = guard.run(super::prepare::run(&mut context)).await?;
    let result = swarm_monitor::run(
        context.session,
        &control,
        super::swarm_policy_run::run(
            context.prompt,
            setup.strategy,
            setup.config,
            context.bus,
            context.output.clone(),
            control.clone(),
        ),
    )
    .await;
    context.session.save().await?;
    let result = result?;
    let result = super::finish::apply(context.session, result).await?;
    guard
        .run(super::handoff::after(
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
