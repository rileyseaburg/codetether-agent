use super::super::CognitionRuntime;
use crate::{AttentionItem, GlobalWorkspace, SwarmGovernance, execution::DecisionReceipt};
impl<T, R> CognitionRuntime<T, R> {
    /// Get the current attention queue.
    pub async fn get_attention_queue(&self) -> Vec<AttentionItem> {
        self.attention_queue.read().await.clone()
    }

    /// Get the shared global workspace.
    pub async fn get_workspace(&self) -> GlobalWorkspace {
        self.workspace.read().await.clone()
    }

    /// Get recorded decision receipts.
    pub async fn get_receipts(&self) -> Vec<DecisionReceipt> {
        self.receipts.read().await.clone()
    }

    /// Get the active governance rules.
    pub async fn get_governance(&self) -> SwarmGovernance {
        self.governance.read().await.clone()
    }
}
