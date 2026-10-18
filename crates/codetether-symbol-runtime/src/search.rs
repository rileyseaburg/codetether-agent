//! Workspace symbol search operations.
use std::collections::HashSet;

use crate::query::query_language;
use codetether_lsp_tool::Host;
use codetether_symbol_search::SymbolEntry;

use codetether_symbol_search::rank;

pub async fn workspace<H: Host>(
    files: &[std::path::PathBuf],
    query: &str,
) -> anyhow::Result<Vec<SymbolEntry>> {
    let mut results = Vec::new();
    let mut last_error = None;
    for file in files {
        match query_language::<H>(file, query).await {
            Ok(found) => results.extend(found),
            Err(error) => last_error = Some(error),
        }
    }
    if results.is_empty() {
        last_error.map_or(Ok(()), Err)?;
    }
    deduplicate(&mut results);
    rank::apply(&mut results, query);
    Ok(results)
}

fn deduplicate(results: &mut Vec<SymbolEntry>) {
    let mut seen = HashSet::new();
    results.retain(|symbol| seen.insert((symbol.name.clone(), symbol.path.clone())));
}
