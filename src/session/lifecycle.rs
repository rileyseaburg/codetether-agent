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

    /// Reattach the process-global bus after loading a session from disk.
    pub(crate) fn attach_global_bus_if_missing(&mut self) {
        if self.bus.is_none() {
            self.bus = crate::bus::global();
        }
    }

    /// Seed session metadata from a loaded [`crate::config::Config`].
    ///
    /// Currently copies [`crate::config::Config::rlm`] into
    /// [`SessionMetadata::rlm`] so RLM compaction and tool-output routing
    /// honour user-configured thresholds, iteration limits, and model
    /// selectors.
    ///
    /// Also attempts to resolve [`RlmConfig::subcall_model`] against the
    /// given provider registry. When resolution succeeds the resolved
    /// provider is cached on [`SessionMetadata`] (not serialised) so
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
        self.metadata.rlm = config.rlm.clone();

        // Resolve subcall_model into a provider, if configured.
        self.metadata.subcall_provider = None;
        self.metadata.subcall_model_name = None;

        if let Some(ref subcall_model_str) = config.rlm.subcall_model
            && let Some(reg) = registry
        {
            match reg.resolve_model(subcall_model_str) {
                Ok((provider, model_name)) => {
                    self.metadata.subcall_provider = Some(provider);
                    self.metadata.subcall_model_name = Some(model_name);
                }
                Err(e) => {
                    tracing::warn!(
                        configured = %subcall_model_str,
                        error = %e,
                        "RLM subcall_model resolution failed; subcalls will use root model"
                    );
                }
            }
        }
    }

    /// Attempt to resolve [`RlmConfig::subcall_model`] against the given
    /// provider registry, storing the result on metadata.
    ///
    /// Called by session helpers right before building an
    /// [`AutoProcessContext`](crate::rlm::router::AutoProcessContext) if
    /// `subcall_provider` is still `None` but `subcall_model` is configured.
    /// This deferred resolution avoids requiring the registry at session
    /// creation time.
    ///
    /// # Errors
    ///
    /// Does **not** return errors — resolution failure is logged.
    pub fn resolve_subcall_provider(&mut self, registry: &crate::provider::ProviderRegistry) {
        if self.metadata.subcall_provider.is_some() {
            return; // Already resolved.
        }
        if let Some(ref subcall_model_str) = self.metadata.rlm.subcall_model {
            match registry.resolve_model(subcall_model_str) {
                Ok((provider, model_name)) => {
                    tracing::debug!(
                        subcall_model = %model_name,
                        "RLM: resolved subcall provider"
                    );
                    self.metadata.subcall_provider = Some(provider);
                    self.metadata.subcall_model_name = Some(model_name);
                }
                Err(e) => {
                    tracing::warn!(
                        configured = %subcall_model_str,
                        error = %e,
                        "RLM subcall_model resolution failed; subcalls will use root model"
                    );
                }
            }
        }
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
