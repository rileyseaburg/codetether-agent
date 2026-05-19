use anyhow::Result;
use streaming_iterator::StreamingIterator;

use super::{AstMatch, AstQueryResult, TreeSitterOracle, query_match};

impl TreeSitterOracle {
    /// Execute a tree-sitter S-expression query.
    pub fn query(&mut self, query_str: &str) -> Result<AstQueryResult> {
        self.parse()?;
        let tree = self.tree.as_ref().expect("tree parsed");
        let query = tree_sitter::Query::new(&tree_sitter_rust::LANGUAGE.into(), query_str)?;
        let matches = collect_matches(&query, tree.root_node(), self.source.as_bytes())?;

        Ok(AstQueryResult {
            query_type: query_str.to_string(),
            matches,
        })
    }
}

fn collect_matches(
    query: &tree_sitter::Query,
    root: tree_sitter::Node<'_>,
    source: &[u8],
) -> Result<Vec<AstMatch>> {
    let mut cursor = tree_sitter::QueryCursor::new();
    let mut stream = cursor.matches(query, root, source);
    let mut results = Vec::new();
    while let Some(match_) = stream.next() {
        results.push(query_match::build(query, match_, source)?);
    }
    Ok(results)
}
