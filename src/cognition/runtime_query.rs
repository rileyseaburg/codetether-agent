//! Read-only accessors for cognition runtime state.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;

use super::beliefs::Belief;
use super::executor::DecisionReceipt;
use super::{
    AttentionItem, CognitionRuntime, GlobalWorkspace, MemorySnapshot, PersonaRuntimeState,
    Proposal, SwarmGovernance, ThoughtEvent,
};
use crate::tool::ToolRegistry;

impl CognitionRuntime {
    /// Whether cognition is enabled by feature flag.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Subscribe to thought events for streaming.
    pub fn subscribe_events(&self) -> broadcast::Receiver<ThoughtEvent> {
        self.event_tx.subscribe()
    }

    /// Set the tool registry for capability-based tool execution.
    pub fn set_tools(&mut self, registry: Arc<ToolRegistry>) {
        self.tools = Some(registry);
    }

    /// Return the most recent memory snapshot, if any.
    pub async fn latest_snapshot(&self) -> Option<MemorySnapshot> {
        self.snapshots.read().await.back().cloned()
    }

    /// Get current beliefs.
    pub async fn get_beliefs(&self) -> HashMap<String, Belief> {
        self.beliefs.read().await.clone()
    }

    /// Get a single belief by ID.
    pub async fn get_belief(&self, id: &str) -> Option<Belief> {
        self.beliefs.read().await.get(id).cloned()
    }

    /// Get the current attention queue.
    pub async fn get_attention_queue(&self) -> Vec<AttentionItem> {
        self.attention_queue.read().await.clone()
    }

    /// Get all proposals.
    pub async fn get_proposals(&self) -> HashMap<String, Proposal> {
        self.proposals.read().await.clone()
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

    /// Get a single persona's runtime state by ID.
    pub async fn get_persona(&self, id: &str) -> Option<PersonaRuntimeState> {
        self.personas.read().await.get(id).cloned()
    }
}
