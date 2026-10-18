//! Prepare the canonical session and worker-specific swarm configuration.
use super::{Session, context::Context};
use codetether_a2a_worker_core::{preparation::session::SessionState, session_protocol::policy};
use codetether_application_providers::worker_swarm::SwarmSetup;
use codetether_native_providers::{ContentPart, Message, Role};
pub(super) async fn run<S: Session>(context: &mut Context<'_, S>) -> anyhow::Result<SwarmSetup> {
    policy::Source::append(
        context.session,
        Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: context.prompt.into(),
            }],
        },
    );
    super::ensure_allowed(context.session)?;
    if policy::Source::needs_title(context.session) {
        context.session.generate_swarm_title().await?;
    }
    let setup = codetether_application_providers::worker_swarm::build::<
        codetether_native_providers::registry::application::Native,
    >(
        SessionState::metadata_mut(context.session),
        context.metadata,
        context.explicit_model.take(),
        context.model_tier,
    )
    .await;
    codetether_swarm_runtime::inline::emit_start(
        &context.output,
        setup.strategy,
        &setup.config,
        context.model_tier,
        context.complexity_hint,
        context.worker_personality,
        context.target_agent_name,
    );
    Ok(setup)
}
