//! tree collect for filesystem inspection.
use super::*;
pub(super) async fn collect(
    path: &Path,
    show_hidden: bool,
    show_size: bool,
    respect_gitignore: bool,
) -> Result<Vec<TreeEntry>> {
    // Read directory and collect entries with their metadata
    let mut entries: Vec<TreeEntry> = Vec::new();

    let mut dir = fs::read_dir(path)
        .await
        .with_context(|| format!("Failed to read directory: {}", path.display()))?;

    loop {
        match dir.next_entry().await {
            Ok(Some(entry)) => {
                let name = entry.file_name().to_string_lossy().to_string();

                // Skip hidden files unless requested
                if !show_hidden && name.starts_with('.') {
                    continue;
                }

                // Skip common ignored directories
                if respect_gitignore && is_pruned_workspace_dir(&name) {
                    continue;
                }

                let file_type = match entry.file_type().await {
                    Ok(ft) => ft,
                    Err(e) => {
                        tracing::warn!(target: "codetether_agent::tool::file_extras",path = %entry.path().display(), error = %e, "Failed to get file type, skipping");
                        continue;
                    }
                };

                let size = if show_size {
                    entry.metadata().await.map(|m| m.len()).unwrap_or(0)
                } else {
                    0
                };

                entries.push(TreeEntry {
                    name,
                    path: entry.path(),
                    is_dir: file_type.is_dir(),
                    size,
                });
            }
            Ok(None) => break, // End of directory
            Err(e) => {
                tracing::warn!(target: "codetether_agent::tool::file_extras",path = %path.display(), error = %e, "Error reading directory entry, continuing");
                continue;
            }
        }
    }

    Ok(entries)
}
