//! extract import references for codebase indexing.

use super::*;

pub(in crate::indexer) fn extract_import_references(
    language: &str,
    line: &str,
    go_import_block: &mut bool,
    imported_modules: &mut Vec<String>,
    imported_symbols: &mut Vec<String>,
) {
    match language {
        "rust" => extract_rust_imports(line, imported_modules, imported_symbols),
        "python" => extract_python_imports(line, imported_modules, imported_symbols),
        "typescript" | "javascript" => {
            extract_script_imports(line, imported_modules, imported_symbols);
        }
        "go" => extract_go_imports(line, go_import_block, imported_modules, imported_symbols),
        _ => {}
    }
}
