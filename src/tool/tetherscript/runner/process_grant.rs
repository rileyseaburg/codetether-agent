//! Filesystem and network authority for plugin subprocesses.

use crate::tool::sandbox::SandboxPolicy;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub(crate) struct ProcessGrant {
    enabled: bool,
    workspace: PathBuf,
    allow_network: bool,
}

impl ProcessGrant {
    pub(crate) fn new(enabled: bool, workspace: PathBuf, allow_network: bool) -> Self {
        Self {
            enabled,
            workspace,
            allow_network,
        }
    }

    pub(super) fn enabled(&self) -> bool {
        self.enabled
    }

    pub(super) fn workspace(&self) -> &Path {
        &self.workspace
    }

    pub(crate) fn policy(&self) -> SandboxPolicy {
        SandboxPolicy {
            allowed_paths: vec![self.workspace.clone()],
            allow_network: self.allow_network,
            allow_exec: true,
            ..SandboxPolicy::default()
        }
    }
}
