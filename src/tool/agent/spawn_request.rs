//! Spawn request extraction for the agent tool.
//!
//! This module validates that spawn-specific fields are present and
//! returns borrowed references for downstream spawn logic. It keeps
//! request parsing separate from policy and persistence.
//!
//! # Examples
//!
//! ```ignore
//! let request = SpawnRequest::from_params(&params)?;
//! assert_eq!(request.name, "reviewer");
//! ```

use super::params::Params;
use anyhow::{Context, Result};
use std::path::PathBuf;

/// Borrowed spawn request fields plus the runtime-owned parent workspace.
///
/// This avoids cloning large strings while spawn validation and session
/// creation run.
///
/// # Examples
///
/// ```ignore
/// let request = SpawnRequest::from_params(&params)?;
/// ```
pub(super) struct SpawnRequest<'a> {
    pub name: &'a str,
    pub instructions: &'a str,
    pub model: &'a str,
    pub parent_workspace: Option<PathBuf>,
}

impl<'a> SpawnRequest<'a> {
    /// Extracts the required spawn fields from the parsed params object.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let request = SpawnRequest::from_params(&params)?;
    /// ```
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
            parent_workspace: params.parent_workspace.clone(),
        })
    }
}
