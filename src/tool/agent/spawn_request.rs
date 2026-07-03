//! Spawn request extraction for the agent tool.
//!
//! This module validates that spawn-specific fields are present and returns
//! borrowed references for downstream spawn logic.

use super::params::Params;
use anyhow::{Context, Result};
use std::path::PathBuf;

/// Borrowed spawn request fields plus the runtime-owned parent workspace.
///
/// This avoids cloning large strings while spawn validation and session
/// creation run.
pub(super) struct SpawnRequest<'a> {
    pub name: &'a str,
    pub instructions: &'a str,
    pub model: &'a str,
    pub ephemeral: bool,
    pub detach: bool,
    pub parent_workspace: Option<PathBuf>,
}

impl<'a> SpawnRequest<'a> {
    /// Extracts the required spawn fields from the parsed params object.
    ///
    /// `model` falls back to the parent's current model (injected by the
    /// runtime as `__ct_current_model`) when the caller omits it, so spawning
    /// works without the LLM having to know a model id.
    pub(super) fn from_params(params: &'a Params) -> Result<Self> {
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
            detach: params.detach_or_default(),
            parent_workspace: params.parent_workspace.clone(),
        })
    }
}
