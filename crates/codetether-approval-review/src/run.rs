//! Run one reviewer agent to completion and produce a verdict.

use std::path::PathBuf;
use std::time::Duration;

use crate::{Host, Run};
use codetether_config::ReviewConfig;

use codetether_approval_core::review::ReviewVerdict;
use codetether_approval_core::review::prompt::{self, ReviewSubject};

/// Review `subject` in `workspace` and return the verdict.
///
/// The reviewer sees only read-only tools. Step and wall-clock caps come
/// from `config`; either expiring yields an `escalate` verdict rather than
/// an error, so the approval overlay always has something to show.
///
/// # Arguments
/// Receives reviewer settings, provider registry, optional session model, workspace
/// and proposed change. The host supplies only native execution operations.
/// # Returns
/// The parsed verdict or an escalation for missing models, errors or timeout.
/// # Examples
/// ```no_run
/// async fn example<H: codetether_approval_review::Host>(registry: &H::Registry) {
///     let verdict = codetether_approval_review::review::<H>(
///         &Default::default(), registry, None, ".".into(), Default::default()).await;
///     assert!(!verdict.reason.is_empty());
/// }
/// ```
pub async fn review<H: Host>(
    config: &ReviewConfig,
    registry: &H::Registry,
    session_model: Option<&str>,
    workspace: PathBuf,
    subject: ReviewSubject,
) -> ReviewVerdict {
    let Some(selector) = config.model.as_deref().or(session_model) else {
        return ReviewVerdict::escalate("no reviewer model configured");
    };
    let (provider, model) = match H::resolve(registry, selector) {
        Ok(resolved) => resolved,
        Err(error) => {
            return ReviewVerdict::escalate(format!("reviewer model unavailable: {error}"));
        }
    };
    let tools = H::tools();
    let definitions = H::definitions(&tools);
    let user = prompt::user(&subject);
    let run = H::execute(Run {
        provider,
        model: &model,
        system: prompt::SYSTEM,
        user: &user,
        definitions,
        tools,
        max_steps: config.max_steps(),
        timeout_secs: config.timeout_secs(),
        id: format!("review-{}", uuid::Uuid::new_v4()),
        workspace,
    });
    let budget = Duration::from_secs(config.timeout_secs() + 5);
    match tokio::time::timeout(budget, run).await {
        Ok(Ok((output, _, _, _))) => codetether_approval_core::review::parse(&output),
        Ok(Err(error)) => ReviewVerdict::escalate(format!("reviewer failed: {error}")),
        Err(_) => ReviewVerdict::escalate("reviewer timed out before reaching a verdict"),
    }
}
