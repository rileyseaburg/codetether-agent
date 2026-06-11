use serde::{Deserialize, Serialize};

/// User approval policy requested by configuration.
///
/// The default asks for approval when a future runtime gate needs elevated
/// permission, which avoids silently granting broad execution.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ApprovalPolicy {
    /// Treat the project as untrusted and require approval for risky actions.
    Untrusted,
    /// Ask only after an operation fails under the current constraints.
    OnFailure,
    /// Ask when an operation needs additional permission.
    #[default]
    OnRequest,
    /// Do not ask for approval during execution.
    Never,
}
