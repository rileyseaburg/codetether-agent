//! analyze file knowledge for codebase indexing.

use super::*;

pub(in crate::indexer) fn analyze_file_knowledge(
    rel_path: &str,
    language: &str,
    _lines: u32,
    _bytes: u64,
    _modified_unix_ms: Option<i64>,
    content: &str,
) -> AnalyzedFileKnowledge {
    let file_id = file_node_id(rel_path);
    let (symbol_nodes, mut imported_modules, mut imported_symbols) =
        scan_file_knowledge(rel_path, language, content);

    imported_modules.sort();
    imported_modules.dedup();
    imported_symbols.sort();
    imported_symbols.dedup();

    let file_node = KnowledgeNode {
        id: file_id,
        kind: "file".to_string(),
        label: rel_path.to_string(),
        file_path: Some(rel_path.to_string()),
        language: Some(language.to_string()),
        symbol_kind: None,
        line: None,
        external: false,
    };

    AnalyzedFileKnowledge {
        file_node,
        symbol_nodes,
        imported_modules,
        imported_symbols,
    }
}
