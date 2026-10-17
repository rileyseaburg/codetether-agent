//! Application binding for shared pending-slot reservation policy.
use super::{host::ApplicationHost, slot::Slot};
use crate::tool::ToolResult;
pub(in crate::tool::agent) async fn reserve(protected: Option<&str>) -> Result<Slot, ToolResult> {
    codetether_agent_residency::reserve::<ApplicationHost>(protected).await
}
