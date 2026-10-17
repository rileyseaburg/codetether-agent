//! extract go import entry for codebase indexing.

use super::*;

pub(in crate::indexer) fn extract_go_import_entry(
    line: &str,
    imported_modules: &mut Vec<String>,
    imported_symbols: &mut Vec<String>,
) {
    let Some(module) = extract_quoted_literal(line) else {
        return;
    };
    imported_modules.push(module.clone());

    let alias = line.split_whitespace().next().unwrap_or("");
    if !alias.is_empty() && !alias.starts_with('"') && !matches!(alias, "_" | ".") {
        if let Some(clean) = sanitize_identifier(alias) {
            imported_symbols.push(clean);
            return;
        }
    }

    if let Some(name) = module.rsplit('/').next().and_then(sanitize_identifier) {
        imported_symbols.push(name);
    }
}
