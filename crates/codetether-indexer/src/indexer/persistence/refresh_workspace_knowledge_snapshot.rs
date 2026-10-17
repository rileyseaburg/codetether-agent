//! refresh workspace knowledge snapshot for codebase indexing.

use super::*;

pub async fn refresh_workspace_knowledge_snapshot(root: &Path) -> Result<PathBuf> {
    let root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let options = BuildOptions {
        include_hidden: false,
        include_embeddings: false,
        max_file_size_bytes: DEFAULT_RUN_KNOWLEDGE_MAX_FILE_SIZE_BYTES,
        ..BuildOptions::default()
    };
    let index = build_index(&root, &options).await?;
    let output_path = default_knowledge_graph_path(&root);

    if let Some(parent) = output_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    let encoded = serde_json::to_string_pretty(&index)?;
    tokio::fs::write(&output_path, encoded).await?;
    Ok(output_path)
}
