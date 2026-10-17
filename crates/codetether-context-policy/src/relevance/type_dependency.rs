//! Shared Dependency contract.
use super::*;

/// CADMAS-CTX dependency axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Dependency {
    /// Single file or a small set of files in the same module.
    Isolated,
    /// Cross-module / multi-file reach.
    Chained,
}
