//! Deterministic AST answers.

use anyhow::Result;

use crate::oracle::{AstPayload, AstResult, FinalPayload, TreeSitterOracle};

/// Answer structural queries directly from tree-sitter evidence.
pub fn answer(content: &str, query: &str, file: String) -> Result<Option<String>> {
    let mut oracle = TreeSitterOracle::new(content.to_string());
    let lower = query.to_lowercase();
    let results = if lower.contains("struct") || lower.contains("field") {
        structs(&mut oracle)?
    } else {
        functions(&mut oracle)?
    };
    let payload = FinalPayload::Ast(AstPayload {
        file,
        query: query.to_string(),
        results,
    });
    Ok(Some(serde_json::to_string(&payload)?))
}

fn functions(oracle: &mut TreeSitterOracle) -> Result<Vec<AstResult>> {
    Ok(oracle
        .get_functions()?
        .into_iter()
        .map(|f| {
            AstResult::function(
                f.name,
                vec![f.params],
                f.return_type,
                Some((f.line, f.line)),
            )
        })
        .collect())
}

fn structs(oracle: &mut TreeSitterOracle) -> Result<Vec<AstResult>> {
    Ok(oracle
        .get_structs()?
        .into_iter()
        .map(|s| AstResult {
            name: s.name,
            args: s.fields,
            return_type: None,
            span: Some((s.line, s.line)),
        })
        .collect())
}
