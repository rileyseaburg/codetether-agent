//! Spawn validation and owner-scoped identity preparation.

use crate::Host;
use codetether_agent_input::SpawnRequest;
use codetether_tool_core::ToolResult;

/// Validated spawn inputs held together with their pending identity claim.
pub(super) struct Prepared {
    warning: Option<String>,
    identity: codetether_spawn_policy::Reservation,
}

impl Prepared {
    /// Return the optional cost warning without consuming the identity claim.
    pub(super) fn warning(&self) -> Option<&str> {
        self.warning.as_deref()
    }

    /// Commit the durable identity handoff and return the warning text.
    pub(super) fn commit(self) -> Option<String> {
        self.identity.commit();
        self.warning
    }
}

/// Validate a request and reserve its owner-scoped display name.
pub(super) async fn run<H: Host>(request: &SpawnRequest<'_>) -> Result<Prepared, ToolResult> {
    let warning = H::validate(request).await?;
    if let Some(text) = &warning {
        tracing::warn!(target: "codetether_agent::tool::agent::spawn::prepare", agent = %request.name, model = %request.model, "{text}");
    }
    let identity = H::reserve_identity(request.name, request.parent_session_id).await?;
    Ok(Prepared { warning, identity })
}
