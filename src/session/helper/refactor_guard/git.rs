use anyhow::Result;
use std::path::{Path, PathBuf};

pub fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

pub async fn head_text(root: &Path, rel: &str) -> Option<String> {
    let args = vec!["show".into(), format!("HEAD:{rel}")];
    let output = crate::tool::git::process::output(root, &args, &[], false)
        .await
        .ok()?;
    output.status.success().then(|| {
        String::from_utf8_lossy(&output.stdout)
            .trim_end_matches('\n')
            .to_string()
    })
}

pub fn existing_sources(paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut out: Vec<_> = paths
        .iter()
        .filter(|path| path.is_file() && super::lines::is_source(path))
        .cloned()
        .collect();
    out.sort();
    out.dedup();
    out
}

pub async fn read_current(path: &Path) -> Result<String> {
    Ok(tokio::fs::read_to_string(path).await?)
}
