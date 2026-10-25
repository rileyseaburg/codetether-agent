//! Atomic working-tree patch application that never touches index or history.

use super::{DeliveryStatus, git};
use anyhow::Result;
use std::path::Path;

/// Apply `patch` to the working tree at `root`, or explain why it was not.
///
/// `git apply` without `--index` edits only working-tree files and is
/// all-or-nothing, so the user's commits, index, and stash are never touched
/// and a partial application cannot happen.
pub(super) fn apply(root: &Path, patch: &[u8]) -> Result<(DeliveryStatus, String)> {
    if patch.is_empty() {
        return Ok((DeliveryStatus::Empty, String::new()));
    }
    let check = git::run(root, &["apply", "--check", "-"], Some(patch))?;
    if !check.status.success() {
        let reverse = git::run(root, &["apply", "--reverse", "--check", "-"], Some(patch))?;
        if reverse.status.success() {
            return Ok((DeliveryStatus::AlreadyPresent, String::new()));
        }
        let detail = String::from_utf8_lossy(&check.stderr).into_owned();
        return Ok((DeliveryStatus::Rejected, detail));
    }
    let applied = git::run(root, &["apply", "-"], Some(patch))?;
    if applied.status.success() {
        return Ok((DeliveryStatus::Applied, String::new()));
    }
    let detail = String::from_utf8_lossy(&applied.stderr).into_owned();
    Ok((DeliveryStatus::Rejected, detail))
}
