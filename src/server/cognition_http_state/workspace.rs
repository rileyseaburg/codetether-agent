//! Native cognition workspace bindings.
use crate::cognition::CognitionRuntime;
use codetether_cognition_core::AttentionItem;
use codetether_cognition_core::GlobalWorkspace;
use codetether_cognition_core::SwarmGovernance;
impl codetether_cognition_api::http::Workspace for CognitionRuntime {
    async fn get_attention_queue(&self) -> Vec<AttentionItem> {
        CognitionRuntime::get_attention_queue(self).await
    }
    async fn get_workspace(&self) -> GlobalWorkspace {
        CognitionRuntime::get_workspace(self).await
    }
    async fn get_governance(&self) -> SwarmGovernance {
        CognitionRuntime::get_governance(self).await
    }
}
