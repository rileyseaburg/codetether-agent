//! Render the index summary selected by the CLI.

use super::*;

pub(super) fn print_summary(index: &CodebaseIndex, output_path: &Path, json: bool) -> Result<()> {
    if json {
        let payload = serde_json::json!({
            "index_path": output_path,
            "root": index.root,
            "generated_at": index.generated_at,
            "embedding_provider": index.embedding_provider,
            "embedding_model": index.embedding_model,
            "stats": index.stats,
            "knowledge_graph": index.knowledge_graph.stats,
        });
        println!("{}", serde_json::to_string_pretty(&payload)?);
    } else {
        text_summary(index, output_path);
    }
    Ok(())
}
