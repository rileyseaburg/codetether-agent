//! Derive parent lineage before applying shared spawn-depth admission.
use super::super::{spawn_request::SpawnRequest, store};
use crate::tool::ToolResult;
pub(super) async fn validate(request: &SpawnRequest<'_>) -> Result<(), ToolResult> {
    let (_, depth) = store::lineage_for_session(request.parent_session_id);
    codetether_spawn_policy::validate_depth(request.name, depth).await
}
