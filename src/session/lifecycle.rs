//! Session construction, agent-name / provenance, and message appending.

use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;

use crate::provenance::ClaimProvenance;
use crate::provider::{Message, Usage};

use super::pages::PageKind;
use super::types::Session;

#[cfg(test)]
#[path = "lifecycle/default_model.rs"]
mod default_model;
#[path = "lifecycle/normalize.rs"]
mod normalize;
#[path = "lifecycle/subcall_model.rs"]
mod subcall_model;

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
        let initial = codetether_session_metadata::lifecycle::InitialSession::new()?;

        Ok(Self {
            id: initial.id,
            title: None,
            created_at: initial.created_at,
            updated_at: initial.updated_at,
            messages: Default::default(),
            pages: Default::default(),
            summary_index: super::index::SummaryIndex::new(),
            tool_uses: Default::default(),
            usage: Usage::default(),
            agent: initial.agent,
            metadata: initial.metadata,
            max_steps: None,
            bus: None,
            storage: Default::default(),
        })
    }

    /// Attach an agent bus for publishing agent thinking/reasoning events.
    pub fn with_bus(mut self, bus: Arc<crate::bus::AgentBus>) -> Self {
        self.bus = Some(bus);
        self
    }

    /// Reattach the process-global bus after loading a session from disk.
    pub(crate) fn attach_global_bus_if_missing(&mut self) {
        if self.bus.is_none() {
            self.bus = crate::bus::global();
        }
    }

    /// Seed session metadata from a loaded [`crate::config::Config`].
    ///
    /// Copies [`crate::config::Config::rlm`] into [`SessionMetadata::rlm`](super::types::SessionMetadata::rlm) so
    /// RLM compaction and tool-output routing honour user-configured
    /// thresholds, iteration limits, and model selectors, and adopts
    /// [`crate::config::Config::default_model`] when the session has no model
    /// yet so the TUI honours `CODETETHER_DEFAULT_MODEL` / `default_model`
    /// exactly like `codetether run`.
    ///
    /// Also attempts to resolve [`RlmConfig::subcall_model`] against the
    /// given provider registry. When resolution succeeds the resolved
    /// provider is cached on [`SessionMetadata`](super::types::SessionMetadata) (not serialised) so
    /// every `AutoProcessContext` built from this session can cheaply
    /// reference it. On failure the subcall provider is left as `None`
    /// and the resolution failure is logged.
    ///
    /// Idempotent: re-applying the same config is a no-op.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_agent::config::Config;
    /// use codetether_agent::session::Session;
    ///
    /// let cfg = Config::default();
    /// let mut session = Session::new().await.unwrap();
    /// session.apply_config(&cfg, None);
    /// assert_eq!(session.metadata.rlm.mode, cfg.rlm.mode);
    /// # });
    /// ```
    pub fn apply_config(
        &mut self,
        config: &crate::config::Config,
        registry: Option<&crate::provider::ProviderRegistry>,
    ) {
        codetether_session_model_policy::apply_config(&mut self.metadata, config, registry);
    }

    /// Set the agent persona owning this session. Also updates the
    /// provenance record so audit logs reflect the new agent.
    pub fn set_agent_name(&mut self, agent_name: impl Into<String>) {
        codetether_session_metadata::lifecycle::set_agent_name(
            &mut self.agent,
            &mut self.metadata,
            agent_name,
        );
    }

    /// Tag the session as having been dispatched by a specific A2A worker
    /// for a specific task.
    pub fn attach_worker_task_provenance(&mut self, worker_id: &str, task_id: &str) {
        codetether_session_metadata::lifecycle::attach_worker_task(
            &mut self.metadata,
            worker_id,
            task_id,
        );
    }

    /// Attach a claim-provenance record to the session's execution
    /// provenance.
    pub fn attach_claim_provenance(&mut self, claim: &ClaimProvenance) {
        codetether_session_metadata::lifecycle::attach_claim(&mut self.metadata, claim);
    }

    /// Append a message to the transcript and bump `updated_at`.
    pub fn add_message(&mut self, message: Message) {
        super::store::append::message(self, message);
        self.updated_at = Utc::now();
    }

    /// Borrow the chat-history transcript as an immutable slice.
    ///
    /// Preferred read path now that [`Self::messages`] is scheduled for
    /// visibility tightening — see the Phase A plan for details. Callers
    /// who need mutable, append-only access should wrap the buffer in a
    /// [`History`](super::history::History) handle obtained via
    /// [`Self::history_mut`].
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_agent::session::Session;
    ///
    /// let session = Session::new().await.unwrap();
    /// assert!(session.history().is_empty());
    /// # });
    /// ```
    pub fn history(&self) -> &[Message] {
        &self.messages
    }

    /// Borrow the per-message page sidecar.
    pub fn pages(&self) -> &[PageKind] {
        &self.pages
    }

    /// Borrow the chat-history transcript as an append-only
    /// [`History`](super::history::History) handle.
    ///
    /// The returned handle can only grow the buffer via
    /// [`History::append`](super::history::History::append). Any mutation
    /// that would violate the Phase A invariant (destructive in-place
    /// rewrite) is not reachable through this API surface.
    pub fn history_mut(&mut self) -> super::history::History<'_> {
        super::history::History::with_pages(&mut self.messages, &mut self.pages)
    }
}
