#![allow(dead_code)]

use super::event::RunEvent;
use anyhow::Result;

pub(in crate::cli::run) fn write_patch_approval_required(
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
