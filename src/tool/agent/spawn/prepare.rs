//! Spawn validation and owner-scoped identity preparation.

use super::super::spawn_request::SpawnRequest;
use crate::tool::ToolResult;

/// Validated spawn inputs held together with their pending identity claim.
pub(super) struct Prepared {
    warning: Option<String>,
    identity: super::identity::Reservation,
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
pub(super) async fn run(request: &SpawnRequest<'_>) -> Result<Prepared, ToolResult> {
    let warning = super::super::spawn_validation::validate_spawn_request(request).await?;
    if let Some(text) = &warning {
        tracing::warn!(agent = %request.name, model = %request.model, "{text}");
    }
    let identity = super::identity::reserve(request.name, request.parent_session_id).await?;
    Ok(Prepared { warning, identity })
}
