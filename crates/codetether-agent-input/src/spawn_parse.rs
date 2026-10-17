//! Required-field validation and parent-model fallback for spawn input.
use crate::{Params, SpawnRequest};
use anyhow::{Context, Result};
use codetether_collaboration_history::ForkTurns;

impl<'a> SpawnRequest<'a> {
    /// Extracts the required spawn fields from the parsed params object.
    ///
    /// `model` falls back to the parent's current model (injected by the
    /// runtime as `__ct_current_model`) when the caller omits it, so spawning
    /// works without the LLM having to know a model id.
    /// # Arguments
    /// * `params` — Parsed agent-tool payload with trusted runtime fields.
    /// # Returns
    /// Borrowed required fields and the cloned workspace path.
    /// # Errors
    /// Returns an error when model, name or instructions are absent, or the
    /// requested history inheritance scope cannot be parsed.
    /// # Examples
    /// See [SpawnRequest] for parsing a payload and deriving its spawn request.
    pub fn from_params(params: &'a Params) -> Result<Self> {
        let model = params
            .model
            .as_deref()
            .or(params._current_model.as_deref())
            .context("model required for spawn (and no parent model available)")?;
        Ok(Self {
            name: params.name.as_deref().context("name required for spawn")?,
            instructions: params
                .instructions
                .as_deref()
                .context("instructions required for spawn")?,
            model,
            ephemeral: params.ephemeral,
            detach: params.detach_for_spawn(),
            parent_workspace: params.parent_workspace.clone(),
            parent_session_id: params.parent_session_id.as_deref(),
            fork_turns: ForkTurns::parse(params.fork_turns.as_deref())?,
        })
    }
}
