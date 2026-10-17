//! default knowledge graph path for codebase indexing.

use super::*;

pub(in crate::indexer) fn default_knowledge_graph_path(root: &Path) -> PathBuf {
    let mut hasher = Sha256::new();
    hasher.update(root.to_string_lossy().as_bytes());
    let digest = hasher.finalize();
    let short = hex::encode(digest);
    let short = &short[..16];

    let base = codetether_paths::data_dir().unwrap_or_else(|| root.join(".codetether-agent"));
    base.join("indexes")
        .join(format!("workspace-knowledge-{short}.json"))
}
