use super::root::trust_root_for;
use super::status::ProjectTrustStatus;
use super::workspace::{canonical_workspace_path, workspace_key};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Filesystem-backed trust store for project-local configuration.
#[derive(Debug, Clone)]
pub struct ProjectTrustStore {
    pub(super) root: PathBuf,
    pub(super) workspace: PathBuf,
    pub(super) key: String,
}

impl ProjectTrustStore {
    /// Create a store for the current process working directory.
    pub fn for_current_workspace() -> Result<Self> {
        let cwd = std::env::current_dir().context("failed to resolve current directory")?;
        Self::for_workspace(cwd)
    }

    /// Create a store for a workspace path using the configured data root.
    pub fn for_workspace(path: impl AsRef<Path>) -> Result<Self> {
        let workspace = canonical_workspace_path(path.as_ref())?;
        let root = trust_root_for(&workspace).context("failed to resolve trust store directory")?;
        Ok(Self::from_parts(root, workspace))
    }

    /// Create a store rooted at `base`, primarily for tests and migrations.
    pub fn with_base(base: impl Into<PathBuf>, workspace: impl AsRef<Path>) -> Result<Self> {
        let workspace = canonical_workspace_path(workspace.as_ref())?;
        Ok(Self::from_parts(base.into(), workspace))
    }

    /// Return the current trust status for this workspace.
    pub fn status(&self) -> ProjectTrustStatus {
        ProjectTrustStatus {
            trusted: self.is_trusted(),
            workspace: self.workspace.clone(),
            key: self.key.clone(),
            record_path: self.record_path(),
        }
    }

    fn from_parts(root: PathBuf, workspace: PathBuf) -> Self {
        let key = workspace_key(&workspace);
        Self {
            root,
            workspace,
            key,
        }
    }
}
