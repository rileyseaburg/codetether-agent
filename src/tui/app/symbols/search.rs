use std::{collections::HashSet, path::Path};

use crate::tool::{Tool, lsp::LspTool};
use crate::tui::symbol_search::SymbolEntry;

use super::{parse, rank};

pub async fn workspace(
    files: &[std::path::PathBuf],
    query: &str,
) -> anyhow::Result<Vec<SymbolEntry>> {
    let mut results = Vec::new();
    let mut last_error = None;
    for file in files {
        match query_language(file, query).await {
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

async fn query_language(file: &Path, query: &str) -> anyhow::Result<Vec<SymbolEntry>> {
    let output = LspTool::new()
        .execute(serde_json::json!({
            "action": "workspaceSymbol", "query": query, "file_path": file,
        }))
        .await?;
    if !output.success {
        anyhow::bail!(output.output);
    }
    Ok(parse::output(&output.output))
}

fn deduplicate(results: &mut Vec<SymbolEntry>) {
    let mut seen = HashSet::new();
    results.retain(|symbol| seen.insert((symbol.name.clone(), symbol.path.clone())));
}
