//! Derived-context adapter for legacy [`Agent::execute`].

use std::sync::Arc;

use anyhow::Result;

use crate::agent::Agent;
use crate::provider::Message;
use crate::session::{Session, derive_context};

/// Build provider-visible history without mutating session history.
pub(super) async fn derive_agent_context(
    agent: &Agent,
    session: &Session,
) -> Result<(String, Vec<Message>)> {
    let model = agent.default_model();
    let tools = agent.tools.definitions();
    let prompt = compose_system_prompt(&agent.system_prompt, session);
    let derived = derive_context(
        session,
        Arc::clone(&agent.provider),
        &model,
        &prompt,
        &tools,
        None,
        None,
    )
    .await?;
    Ok((prompt, derived.messages))
}

/// Append goal governance to the agent's base system prompt.
pub(super) fn compose_system_prompt(base: &str, session: &Session) -> String {
    let log = match crate::session::tasks::TaskLog::for_session(&session.id) {
        Ok(l) => l,
        Err(_) => return base.to_string(),
    };
    let events = log.read_all_blocking().unwrap_or_default();
    let state = crate::session::tasks::TaskState::from_log(&events);
    match crate::session::tasks::governance_block(&state) {
        Some(block) => format!("{base}\n\n{block}"),
        None => base.to_string(),
    }
}
