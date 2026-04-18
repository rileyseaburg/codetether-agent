//! Session construction, agent-name / provenance, and message appending.

use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use uuid::Uuid;

use crate::agent::ToolUse;
use crate::provenance::{ClaimProvenance, ExecutionProvenance};
use crate::provider::{Message, Usage};

use super::types::{Session, SessionMetadata};

impl Session {
    /// Create a new empty session rooted at the current working directory.
    ///
    /// # Errors
    ///
    /// Returns an error if the current working directory cannot be resolved.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_agent::session::Session;
    ///
    /// let session = Session::new().await.unwrap();
    /// assert!(!session.id.is_empty());
    /// assert_eq!(session.agent, "build");
    /// assert!(session.messages.is_empty());
    /// # });
    /// ```
    pub async fn new() -> Result<Self> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let provenance = Some(ExecutionProvenance::for_session(&id, "build"));

        Ok(Self {
            id,
            title: None,
            created_at: now,
            updated_at: now,
            messages: Vec::new(),
            tool_uses: Vec::<ToolUse>::new(),
            usage: Usage::default(),
            agent: "build".to_string(),
            metadata: SessionMetadata {
                directory: Some(std::env::current_dir()?),
                provenance,
                ..Default::default()
            },
            max_steps: None,
            bus: None,
        })
    }

    /// Attach an agent bus for publishing agent thinking/reasoning events.
    pub fn with_bus(mut self, bus: Arc<crate::bus::AgentBus>) -> Self {
        self.bus = Some(bus);
        self
    }

    /// Set the agent persona owning this session. Also updates the
    /// provenance record so audit logs reflect the new agent.
    pub fn set_agent_name(&mut self, agent_name: impl Into<String>) {
        let agent_name = agent_name.into();
        self.agent = agent_name.clone();
        if let Some(provenance) = self.metadata.provenance.as_mut() {
            provenance.set_agent_name(&agent_name);
        }
    }

    /// Tag the session as having been dispatched by a specific A2A worker
    /// for a specific task.
    pub fn attach_worker_task_provenance(&mut self, worker_id: &str, task_id: &str) {
        if let Some(provenance) = self.metadata.provenance.as_mut() {
            provenance.apply_worker_task(worker_id, task_id);
        }
    }

    /// Attach a claim-provenance record to the session's execution
    /// provenance.
    pub fn attach_claim_provenance(&mut self, claim: &ClaimProvenance) {
        if let Some(provenance) = self.metadata.provenance.as_mut() {
            provenance.apply_claim(claim);
        }
    }

    /// Append a message to the transcript and bump `updated_at`.
    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        self.updated_at = Utc::now();
    }
}
