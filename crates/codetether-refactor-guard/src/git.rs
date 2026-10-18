//! Read changed source files and their committed baselines.
use anyhow::Result;
use std::path::{Path, PathBuf};
use tokio::process::Command;

pub(super) fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

pub(super) async fn head_text(root: &Path, rel: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["show", &format!("HEAD:{rel}")])
        .current_dir(root)
        .output()
        .await
        .ok()?;
    output.status.success().then(|| {
        String::from_utf8_lossy(&output.stdout)
            .trim_end_matches('\n')
            .to_string()
    })
}

pub(super) fn existing_sources(paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut out: Vec<_> = paths
        .iter()
        .filter(|path| path.is_file() && super::lines::is_source(path))
        .cloned()
        .collect();
    out.sort();
    out.dedup();
    out
}

pub(super) async fn read_current(path: &Path) -> Result<String> {
    Ok(tokio::fs::read_to_string(path).await?)
}
