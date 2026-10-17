use serde::{Deserialize, Serialize};

/// Filesystem sandbox mode requested by configuration.
///
/// Defaults to read-only so config parsing does not imply write access until
/// runtime code explicitly opts into a broader mode.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum SandboxMode {
    /// Permit reads only.
    #[default]
    ReadOnly,
    /// Permit writes inside the active workspace.
    WorkspaceWrite,
    /// Disable filesystem sandboxing.
    DangerFullAccess,
}
