//! Discover symbols and imports within a source file.

use super::*;

pub(in crate::indexer) fn scan_file_knowledge(
    rel_path: &str,
    language: &str,
    content: &str,
) -> (Vec<KnowledgeNode>, Vec<String>, Vec<String>) {
    let mut symbol_nodes = Vec::new();
    let mut imported_modules = Vec::new();
    let mut imported_symbols = Vec::new();
    let mut seen_symbols: HashSet<(String, u32, String)> = HashSet::new();
    let mut go_import_block = false;

    for (idx, raw_line) in content.lines().enumerate() {
        let line_no = idx as u32 + 1;
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some((symbol_kind, name)) = extract_symbol_definition(language, line) {
            let key = (name.clone(), line_no, symbol_kind.to_string());
            if seen_symbols.insert(key) {
                symbol_nodes.push(KnowledgeNode {
                    id: symbol_node_id(rel_path, &name, line_no),
                    kind: "symbol".to_string(),
                    label: name,
                    file_path: Some(rel_path.to_string()),
                    language: Some(language.to_string()),
                    symbol_kind: Some(symbol_kind.to_string()),
                    line: Some(line_no),
                    external: false,
                });
            }
        }

        extract_import_references(
            language,
            line,
            &mut go_import_block,
            &mut imported_modules,
            &mut imported_symbols,
        );
    }

    (symbol_nodes, imported_modules, imported_symbols)
}
