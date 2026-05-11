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
    pub parent_workspace: Option<PathBuf>,
}

impl<'a> SpawnRequest<'a> {
    /// Extracts the required spawn fields from the parsed params object.
    pub(super) fn from_params(params: &'a Params) -> Result<Self> {
        Ok(Self {
            name: params.name.as_deref().context("name required for spawn")?,
            instructions: params
                .instructions
                .as_deref()
                .context("instructions required for spawn")?,
            model: params
                .model
                .as_deref()
                .context("model required for spawn")?,
            ephemeral: params.ephemeral,
            parent_workspace: params.parent_workspace.clone(),
        })
    }
}
