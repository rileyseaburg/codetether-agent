//! Patch approval output.
#![allow(dead_code)]

use super::event::RunEvent;
use anyhow::Result;

/// Write a patch approval required record to standard output.
/// # Arguments
/// * `timestamp_ms` - Event timestamp in milliseconds.
/// * `approval_id` - Approval request identity.
/// * `patch_id` - Patch identity.
/// # Returns
/// Unit after the serialized record and newline are written.
/// # Errors
/// Returns JSON serialization or output I/O errors.
/// # Examples
/// ```no_run
/// # fn main() -> anyhow::Result<()> {
/// codetether_run_jsonl::write_patch_approval_required("approval-1", "patch-1", 0)?;
/// # Ok(()) }
/// ```
pub fn write_patch_approval_required(
    approval_id: &str,
    patch_id: &str,
    timestamp_ms: u64,
) -> Result<()> {
    super::writer::write_stdout(&RunEvent::PatchApprovalRequired {
        approval_id,
        patch_id,
        timestamp_ms,
    })
}
