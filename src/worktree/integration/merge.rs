use super::{commit, delta, git, outcome::Outcome};
use anyhow::{Result, bail};
use std::path::Path;

pub(super) fn merge(repo: &Path, branch: &str) -> Result<Outcome> {
    let expected = delta::expected(repo, branch)?;
    if expected.is_empty() {
        return Ok(Outcome::Merged(expected));
    }
    let output = git::output(repo, &["merge", "--no-ff", "--no-commit", branch])?;
    if !output.status.success() {
        if has_conflict(&output) {
            return Ok(Outcome::Conflicted(git::lines(
                repo,
                &["diff", "--name-only", "--diff-filter=U"],
            )?));
        }
        bail!(
            "integration merge failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    commit::create(repo, branch)?;
    let actual = delta::actual(repo)?;
    if actual != expected {
        bail!("integration delta mismatch: expected {expected:?}, found {actual:?}");
    }
    Ok(Outcome::Merged(actual))
}

fn has_conflict(output: &std::process::Output) -> bool {
    String::from_utf8_lossy(&output.stdout).contains("CONFLICT")
        || String::from_utf8_lossy(&output.stderr).contains("CONFLICT")
}
