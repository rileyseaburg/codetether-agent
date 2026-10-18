//! Optional caller-owned continuation hook after built-in guard rules.
use crate::{GuardReport, GuardViolation};
use anyhow::Result;
use async_trait::async_trait;
use std::path::Path;
/// Native extension evaluated after the built-in report is populated.
/// # Implementors
/// Implement using the application's optional TetherScript runtime.
#[async_trait]
pub trait Hook: Sync {
    /// Return an additional violation, or None to continue without one.
    /// # Errors
    /// Propagates native hook execution and decoding failures.
    async fn apply(&self, root: &Path, report: &GuardReport) -> Result<Option<GuardViolation>>;
}
