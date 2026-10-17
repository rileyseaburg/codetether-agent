//! extract go imports for codebase indexing.

use super::*;

pub(in crate::indexer) fn extract_go_imports(
    line: &str,
    go_import_block: &mut bool,
    imported_modules: &mut Vec<String>,
    imported_symbols: &mut Vec<String>,
) {
    let trimmed = line.trim();

    if *go_import_block {
        if trimmed == ")" {
            *go_import_block = false;
            return;
        }
        extract_go_import_entry(trimmed, imported_modules, imported_symbols);
        return;
    }

    if trimmed == "import (" {
        *go_import_block = true;
        return;
    }

    if let Some(rest) = trimmed.strip_prefix("import ") {
        extract_go_import_entry(rest.trim(), imported_modules, imported_symbols);
    }
}
