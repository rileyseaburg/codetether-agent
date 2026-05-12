use std::collections::HashMap;

/// Result of tree-sitter AST query.
#[derive(Debug, Clone, PartialEq)]
pub struct AstQueryResult {
    /// Query type that was executed.
    pub query_type: String,
    /// Matched nodes with their captures.
    pub matches: Vec<AstMatch>,
}

/// A single match from an AST query.
#[derive(Debug, Clone, PartialEq)]
pub struct AstMatch {
    /// Line number, 1-indexed.
    pub line: usize,
    /// Column, 1-indexed.
    pub column: usize,
    /// Captured nodes keyed by capture name.
    pub captures: HashMap<String, String>,
    /// Full text of the first captured node.
    pub text: String,
}
