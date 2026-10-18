//! Adapt the optional native TetherScript extension to the shared guard.
use anyhow::Result;
use async_trait::async_trait;
use codetether_refactor_guard::{GuardReport, GuardViolation};
use std::path::Path;
pub(super) struct NativeHook;
#[async_trait]
impl codetether_refactor_guard::Hook for NativeHook {
    async fn apply(&self, root: &Path, report: &GuardReport) -> Result<Option<GuardViolation>> {
        super::plugin::apply(root, report).await
    }
}
