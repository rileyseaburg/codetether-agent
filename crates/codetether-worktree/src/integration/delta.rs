use super::git;
use anyhow::Result;
use std::{collections::BTreeSet, path::Path};

pub(super) fn expected(repo: &Path, branch: &str) -> Result<BTreeSet<String>> {
    let cherry = git::lines(repo, &["cherry", "HEAD", branch])?;
    let mut files = BTreeSet::new();
    for line in cherry.into_iter().filter(|line| line.starts_with('+')) {
        let commit = line.split_whitespace().nth(1).unwrap_or_default();
        files.extend(git::lines(
            repo,
            &["show", "--pretty=format:", "--name-only", commit],
        )?);
    }
    Ok(files)
}

pub(super) fn actual(repo: &Path) -> Result<BTreeSet<String>> {
    Ok(
        git::lines(repo, &["diff", "--name-only", "HEAD^1", "HEAD"])?
            .into_iter()
            .collect(),
    )
}
