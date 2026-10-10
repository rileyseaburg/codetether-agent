use super::git;
use anyhow::{Result, bail};
use std::path::Path;

pub(super) fn fast_forward(target: &Path, commit: &str) -> Result<()> {
    let output = git::output(target, &["merge", "--ff-only", commit])?;
    if !output.status.success() {
        bail!(
            "verified merge could not fast-forward target: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}
