//! Read-only accessors for cognition runtime state.

use crate::Host;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;

use super::{
    AttentionItem, CognitionRuntime, GlobalWorkspace, MemorySnapshot, PersonaRuntimeState,
    Proposal, SwarmGovernance, ThoughtEvent,
};
use codetether_cognition_core::beliefs::Belief;
use codetether_cognition_core::execution::DecisionReceipt;

impl<H: Host> CognitionRuntime<H> {
    /// Whether cognition is enabled by feature flag.
    pub fn is_enabled(&self) -> bool {
        self.state.is_enabled()
    }

    /// Subscribe to thought events for streaming.
    pub fn subscribe_events(&self) -> broadcast::Receiver<ThoughtEvent> {
        self.state.subscribe_events()
    }

    /// Set the tool registry for capability-based tool execution.
    pub fn set_tools(&mut self, registry: Arc<H::Registry>) {
        self.state.set_tools(registry)
    }

    /// Return the most recent memory snapshot, if any.
    pub async fn latest_snapshot(&self) -> Option<MemorySnapshot> {
        self.state.latest_snapshot().await
    }

    /// Get current beliefs.
    pub async fn get_beliefs(&self) -> HashMap<String, Belief> {
        self.state.get_beliefs().await
    }

    /// Get a single belief by ID.
    pub async fn get_belief(&self, id: &str) -> Option<Belief> {
        self.state.get_belief(id).await
    }

    /// Get the current attention queue.
    pub async fn get_attention_queue(&self) -> Vec<AttentionItem> {
        self.state.get_attention_queue().await
    }

    /// Get all proposals.
    pub async fn get_proposals(&self) -> HashMap<String, Proposal> {
        self.state.get_proposals().await
    }

    /// Get the shared global workspace.
    pub async fn get_workspace(&self) -> GlobalWorkspace {
        self.state.get_workspace().await
    }

    /// Get recorded decision receipts.
    pub async fn get_receipts(&self) -> Vec<DecisionReceipt> {
        self.state.get_receipts().await
    }

    /// Get the active governance rules.
    pub async fn get_governance(&self) -> SwarmGovernance {
        self.state.get_governance().await
    }

    /// Get a single persona's runtime state by ID.
    pub async fn get_persona(&self, id: &str) -> Option<PersonaRuntimeState> {
        self.state.get_persona(id).await
    }
}
