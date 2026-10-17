//! extract script imports for codebase indexing.

use super::*;

pub(in crate::indexer) fn extract_script_imports(
    line: &str,
    imported_modules: &mut Vec<String>,
    imported_symbols: &mut Vec<String>,
) {
    let trimmed = line.trim();
    let is_module_import = trimmed.starts_with("import ")
        || (trimmed.starts_with("export ") && trimmed.contains(" from "));
    if !is_module_import && !trimmed.contains("require(") {
        return;
    }

    if let Some(module) = extract_quoted_literal(trimmed) {
        imported_modules.push(module.clone());
        if let Some(name) = module.rsplit('/').next().and_then(sanitize_identifier) {
            imported_symbols.push(name);
        }
    }

    if let Some((before_from, _)) = trimmed.split_once(" from ") {
        if let Some((default_import, _)) = before_from
            .trim_start_matches("import ")
            .trim_start_matches("export ")
            .split_once(',')
        {
            let default_import = default_import.trim();
            if !default_import.is_empty() && !default_import.starts_with('{') {
                if let Some(name) = sanitize_identifier(default_import) {
                    imported_symbols.push(name);
                }
            }
        }
    }

    if let Some(braced) = extract_braced_section(trimmed) {
        for name in braced.split(',') {
            let name = name.trim();
            let alias_free = name.split(" as ").next().unwrap_or(name).trim();
            let alias_free = alias_free.trim_start_matches("type ").trim();
            if let Some(clean) = sanitize_identifier(alias_free) {
                imported_symbols.push(clean);
            }
        }
    }
}
