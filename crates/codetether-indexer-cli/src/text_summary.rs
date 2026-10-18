//! Human-readable index summary.

use super::*;

pub(super) fn text_summary(index: &CodebaseIndex, output_path: &Path) {
    println!("# Codebase Index Built\n");
    println!("- Root: {}", index.root);
    println!("- Output: {}", output_path.display());
    println!(
        "- Embeddings: {}/{}",
        index.embedding_provider, index.embedding_model
    );
    println!("- Indexed files: {}", index.stats.indexed_files);
    println!("- Embedded files: {}", index.stats.embedded_files);
    println!(
        "- Embedding dimensions: {}",
        index.stats.embedding_dimensions
    );
    println!("- Total lines: {}", index.stats.total_lines);
    println!("- Total bytes: {}", index.stats.total_bytes);
    println!(
        "- Knowledge graph: {} nodes / {} edges",
        index.knowledge_graph.nodes.len(),
        index.knowledge_graph.edges.len()
    );
    if !index.stats.language_counts.is_empty() {
        let mut langs: Vec<_> = index.stats.language_counts.iter().collect();
        langs.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
        println!("\nTop languages:");
        for (lang, count) in langs.into_iter().take(10) {
            println!("- {}: {} files", lang, count);
        }
    }
}
