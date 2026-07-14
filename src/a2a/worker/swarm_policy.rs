//! Swarm execution policy for worker tasks.

use std::sync::Arc;

use anyhow::Result;

use crate::{
    bus::AgentBus,
    provider::{ContentPart, Message, Role},
    session::Session,
};

use super::swarm_setup::SwarmSetup;

mod access;
mod finish;
mod swarm_policy_run;
mod swarm_policy_start;

pub(super) async fn execute_swarm_with_policy(
    session: &mut Session,
    prompt: &str,
    model_tier: Option<&str>,
    explicit_model: Option<String>,
    metadata: &serde_json::Map<String, serde_json::Value>,
    complexity_hint: Option<&str>,
    worker_personality: Option<&str>,
    target_agent_name: Option<&str>,
    bus: Option<&Arc<AgentBus>>,
    output_callback: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) -> Result<(crate::session::SessionResult, bool)> {
    session.add_delegated_message(Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: prompt.to_string(),
        }],
    });
    access::ensure(session)?;
    if session.title.is_none() {
        session.generate_title().await?;
    }
    let SwarmSetup { config, strategy } =
        super::build_swarm_setup(session, metadata, explicit_model, model_tier).await;
    swarm_policy_start::emit(
        &output_callback,
        strategy,
        &config,
        model_tier,
        complexity_hint,
        worker_personality,
        target_agent_name,
    );
    let result = swarm_policy_run::run(prompt, strategy, config, bus, output_callback).await?;
    finish::apply(session, result).await
}
