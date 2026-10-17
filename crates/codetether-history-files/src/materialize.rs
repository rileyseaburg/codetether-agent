//! Write changed transcript turns and remove stale files from their history directory.
use crate::{
    paths::{format_turn_path, history_dir},
    render_turn, role_label,
};
use crate::{
    prune::{file_name, prune_stale_turn_files},
    write::write_turn_if_changed,
};
use anyhow::{Context, Result};
use codetether_provider_core::Message;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};
use tokio::fs;

/// Materialize an entire session transcript, then prune stale regular files.
///
/// # Arguments
/// * `session_id` - Durable session identifier.
/// * `workspace` - Optional workspace used to locate its data directory.
/// * `messages` - Complete canonical transcript in turn order.
/// # Returns
/// Ordered paths to the materialized turns.
/// # Errors
/// Returns directory, file-read, file-write or stale-file cleanup errors.
/// # Examples
/// ```rust,no_run
/// # async fn example()->anyhow::Result<()> {
/// let paths=codetether_history_files::materialize_history("session",None,&[]).await?;
/// assert!(paths.is_empty());
/// # Ok(()) }
/// ```
pub async fn materialize_history(
    session_id: &str,
    workspace: Option<&Path>,
    messages: &[Message],
) -> Result<Vec<PathBuf>> {
    let dir = history_dir(session_id, workspace)?;
    fs::create_dir_all(&dir)
        .await
        .with_context(|| format!("create {}", dir.display()))?;

    let mut paths = Vec::with_capacity(messages.len());
    let mut expected = HashSet::with_capacity(messages.len());
    for (idx, msg) in messages.iter().enumerate() {
        let path = format_turn_path(&dir, idx, role_label(&msg.role));
        let body = render_turn(msg);
        write_turn_if_changed(&path, &body).await?;
        expected.insert(file_name(&path)?);
        paths.push(path);
    }
    prune_stale_turn_files(&dir, &expected).await?;
    Ok(paths)
}
