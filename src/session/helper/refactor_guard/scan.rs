use anyhow::Result;
use std::path::{Path, PathBuf};

use super::types::{FileStatus, GuardFile};

pub async fn files(root: &Path, paths: &[PathBuf]) -> Result<Vec<GuardFile>> {
    let config = super::config::GuardConfig::load(root);
    let mut out = Vec::new();
    for path in super::git::existing_sources(paths) {
        let rel = super::git::relative(root, &path);
        let Some(limit) = config.limit_for(&rel) else {
            continue;
        };
        let new_text = super::git::read_current(&path).await?;
        let old_text = super::git::head_text(root, &rel).await;
        let old_code_lines = old_text.as_deref().map(super::lines::code_lines);
        let new_code_lines = super::lines::code_lines(&new_text);
        let wrapper_target = super::wrapper::target(&rel, &new_text);
        let status = if old_text.is_some() {
            FileStatus::Modified
        } else {
            FileStatus::Added
        };
        out.push(GuardFile {
            path: rel,
            status,
            old_code_lines,
            new_code_lines,
            limit,
            wrapper_target,
            old_text,
            new_text,
        });
    }
    Ok(out)
}
