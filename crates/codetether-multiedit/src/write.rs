//! Write each validated path once using its final accumulated content.
use anyhow::Result;
use std::{collections::HashMap, path::PathBuf};
use tokio::fs;
pub(super) async fn write(
    validated: &[(PathBuf, String, String)],
    content_cache: &HashMap<PathBuf, String>,
) -> Result<(HashMap<PathBuf, bool>, Vec<String>)> {
    let mut written: HashMap<PathBuf, bool> = HashMap::new();
    let mut write_errors: Vec<String> = Vec::new();

    for (path, _original, _new) in validated {
        if written.contains_key(path) {
            continue;
        }
        let final_content = content_cache
            .get(path)
            .ok_or_else(|| anyhow::anyhow!("path {path:?} not found in content cache"))?;
        match fs::write(path, final_content).await {
            Ok(()) => {
                written.insert(path.clone(), true);
            }
            Err(e) => {
                write_errors.push(format!("{}: write failed: {e}", path.display()));
                written.insert(path.clone(), false);
            }
        }
    }

    Ok((written, write_errors))
}
