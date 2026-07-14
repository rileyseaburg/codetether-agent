//! Child-session preparation for an agent message.

use super::super::{helpers, session_factory, store};
use crate::session::Session;
use anyhow::{Context, Result};

/// Load a child session and apply its parent's current access policy.
pub(super) async fn load(name: &str, params: &helpers::Params) -> Result<Session> {
    let mut session = store::get_for_parent(name, params.parent_session_id.as_deref())
        .map(|entry| entry.session)
        .context(format!("Agent @{name} not found"))?;
    session.metadata.inherited_prior_context_allowed = Some(
        session_factory::parent_prior_context_allowed(
            params.parent_prior_context_allowed,
            params.parent_session_id.as_deref(),
        )
        .await,
    );
    if session.bus.is_none() {
        session.bus = crate::bus::global();
    }
    Ok(session)
}
