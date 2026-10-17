//! Shared ToolUse contract.
use super::*;

/// CADMAS-CTX tool-use axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolUse {
    No,
    Yes,
}
