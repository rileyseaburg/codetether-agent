//! Prepare the canonical session and worker-specific swarm configuration.

use super::context::Context;
use crate::provider::{ContentPart, Message, Role};
use codetether_application_providers::worker_swarm::SwarmSetup;

pub(super) async fn run(context: &mut Context<'_>) -> anyhow::Result<SwarmSetup> {
    context.session.add_delegated_message(Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: context.prompt.into(),
        }],
    });
    super::access::ensure(context.session)?;
    if context.session.title.is_none() {
        context.session.generate_title().await?;
    }
    let setup = codetether_application_providers::worker_swarm::build::<
        crate::provider::registry::application::Native,
    >(
        &mut context.session.metadata,
        context.metadata,
        context.explicit_model.take(),
        context.model_tier,
    )
    .await;
    super::swarm_policy_start::emit(
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
