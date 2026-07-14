//! Proactive RLM preparation at provider-step boundaries.

use std::sync::Arc;

use anyhow::Result;

use crate::provider::{Message, Provider};
use crate::session::Session;

pub(super) fn run(session: &Session, provider: Arc<dyn Provider>, model: &str) {
    crate::session::index_produce::proactive::prepare(session, provider, model);
}

pub(crate) fn done(session: &Session, provider: &Arc<dyn Provider>, model: &str) -> Result<()> {
    run(session, Arc::clone(provider), model);
    Ok(())
}

pub(crate) async fn prepare_messages(
    output: &mut Vec<Message>,
    session: &Session,
    derived: &mut [Message],
) {
    if let Some(message) = crate::session::index::recall::prefetch::message(session).await {
        output.push(message);
    }
    super::super::rlm_background::resolve_pending(derived);
}
