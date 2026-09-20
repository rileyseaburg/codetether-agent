//! Run one reviewer agent to completion and produce a verdict.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use crate::config::ReviewConfig;
use crate::provider::ProviderRegistry;
use crate::swarm::run_agent_loop;

use super::prompt::{self, ReviewSubject};
use super::verdict::ReviewVerdict;

/// Review `subject` in `workspace` and return the verdict.
///
/// The reviewer sees only read-only tools. Step and wall-clock caps come
/// from `config`; either expiring yields an `escalate` verdict rather than
/// an error, so the approval overlay always has something to show.
pub async fn review(
    config: &ReviewConfig,
    registry: &ProviderRegistry,
    session_model: Option<&str>,
    workspace: PathBuf,
    subject: ReviewSubject,
) -> ReviewVerdict {
    let Some(selector) = config.model.as_deref().or(session_model) else {
        return ReviewVerdict::escalate("no reviewer model configured");
    };
    let (provider, model) = match registry.resolve_model(selector) {
        Ok(resolved) => resolved,
        Err(error) => {
            return ReviewVerdict::escalate(format!("reviewer model unavailable: {error}"));
        }
    };
    let tools = Arc::new(super::tools::read_only_tools());
    let definitions = tools.definitions();
    let user = prompt::user(&subject);
    let run = run_agent_loop(
        provider,
        &model,
        prompt::SYSTEM,
        &user,
        definitions,
        tools,
        config.max_steps(),
        config.timeout_secs(),
        None,
        format!("review-{}", uuid::Uuid::new_v4()),
        None,
        Some(workspace),
    );
    let budget = Duration::from_secs(config.timeout_secs() + 5);
    match tokio::time::timeout(budget, run).await {
        Ok(Ok((output, _, _, _))) => super::parse::parse(&output),
        Ok(Err(error)) => ReviewVerdict::escalate(format!("reviewer failed: {error}")),
        Err(_) => ReviewVerdict::escalate("reviewer timed out before reaching a verdict"),
    }
}
