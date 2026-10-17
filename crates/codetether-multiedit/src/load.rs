//! Read original content or reuse accumulated content for a repeated path.
use super::op::EditOp;
use std::{collections::HashMap, path::PathBuf};
use tokio::fs;
pub(super) async fn load(
    path: &PathBuf,
    op: &EditOp,
    i: usize,
    content_cache: &HashMap<PathBuf, String>,
) -> Result<String, String> {
    if let Some(cached) = content_cache.get(path) {
        Ok(cached.clone())
    } else if path.exists() {
        match fs::read_to_string(path).await {
            Ok(c) => Ok(c),
            Err(e) => {
                return Err(format!("edits[{i}] {}: cannot read file: {e}", op.file));
            }
        }
    } else {
        Err(format!("edits[{i}] {}: file does not exist", op.file))
    }
}
