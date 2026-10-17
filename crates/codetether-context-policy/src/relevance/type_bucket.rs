//! Shared Bucket contract.
use super::*;

/// Coarse context bucket — CADMAS-CTX Section 3.1.
///
/// Start with 3–4 active cells in practice (the paper shows over-
/// bucketing hurts — bias-variance collapses at 12 cells on GAIA).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bucket {
    pub difficulty: Difficulty,
    pub dependency: Dependency,
    pub tool_use: ToolUse,
}
