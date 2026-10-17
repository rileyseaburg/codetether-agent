use serde::{Deserialize, Serialize};

/// Trust level for project-local configuration.
///
/// Untrusted is the default so project-local hooks or rule files can be gated
/// by this value later without granting trust to existing repositories.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ProjectTrustLevel {
    /// Project-local configuration has not been explicitly trusted.
    #[default]
    Untrusted,
    /// Project-local configuration may be treated as user-trusted.
    Trusted,
}
