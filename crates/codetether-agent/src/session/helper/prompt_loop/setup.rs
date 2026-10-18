//! Assemble the native prompt runner from shared initialization results.
use super::Runner;
use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::mpsc;
mod host;
struct Native<'a> {
    session: &'a mut Session,
    events: Option<&'a mpsc::Sender<SessionEvent>>,
    registry: &'a ProviderRegistry,
}
/// Initialize the native runner through shared turn preparation.
/// # Errors
/// Returns initial provider/model selection errors.
pub(crate) async fn initialize<'a>(
    session: &'a mut Session,
    events: Option<mpsc::Sender<SessionEvent>>,
    registry: Arc<ProviderRegistry>,
) -> Result<Runner<'a>> {
    let parts = codetether_prompt_loop::setup::initialize(&mut Native {
        session,
        events: events.as_ref(),
        registry: &registry,
    })
    .await?;
    Ok(Runner {
        inner: codetether_prompt_loop::RunnerState {
            session,
            events,
            registry,
            lease_owner: parts.lease_owner,
            model: parts.model,
            workspace: parts.workspace,
            progress: parts.progress,
            router: parts.router,
            subagents: parts.subagents,
        },
    })
}
