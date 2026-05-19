//! FINAL(JSON) schema for oracle-eligible RLM outputs.
//!
//! This module defines the structured JSON schema that RLM FINAL() outputs must conform to
//! for deterministic oracle verification. The `kind` field drives validator routing.
//!
//! # Schema Types
//!
//! - **Grep**: Pattern-match results (line numbers, text content)
//! - **Ast**: Structural AST query results (function signatures, struct fields)
//! - **Semantic**: Free-form text answers (unverifiable - stored but not golden)
//!
//! # Usage
//!
//! ```ignore
//! use codetether_agent::rlm::oracle::schema::{FinalPayload, GrepPayload, AstPayload};
//!
//! // Parse a FINAL() JSON output
//! let payload = FinalPayload::parse(r#"{"kind": "grep", "file": "src/main.rs", ...}"#)?;
//!
//! match payload {
//!     FinalPayload::Grep(grep) => { /* verify with GrepOracle */ }
//!     FinalPayload::Ast(ast) => { /* verify with TreeSitterOracle */ }
//!     FinalPayload::Semantic(_) => { /* cannot verify - skip */ }
//!     FinalPayload::Malformed { .. } => { /* log and skip */ }
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

/// The top-level FINAL() payload envelope.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum FinalPayload {
    /// Grep/pattern-match query results
    Grep(GrepPayload),
    /// AST/structural query results
    Ast(AstPayload),
    /// Semantic/free-form text (unverifiable)
    Semantic(SemanticPayload),
    /// Malformed JSON that couldn't be parsed
    Malformed {
        /// The raw string that failed to parse
        raw: String,
        /// Error message from parsing attempt
        error: String,
    },
}

impl FinalPayload {
    /// Parse a JSON string into a FinalPayload.
    ///
    /// Returns `FinalPayload::Malformed` if parsing fails.
    pub fn parse(json_str: &str) -> Self {
        let trimmed = json_str.trim();

        // Try to parse as JSON
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(trimmed);

        match parsed {
            Ok(value) => {
                // Try to deserialize into our enum
                match serde_json::from_value::<FinalPayload>(value.clone()) {
                    Ok(payload) => payload,
                    Err(e) => {
                        // JSON is valid but doesn't match our schema
                        // Check if it has a "kind" field we can use
                        if let Some(kind) = value.get("kind").and_then(|k| k.as_str()) {
                            match kind {
                                "grep" => serde_json::from_value(value).unwrap_or_else(|e2| {
                                    FinalPayload::Malformed {
                                        raw: trimmed.to_string(),
                                        error: format!("GrepPayload parse error: {}", e2),
                                    }
                                }),
                                "ast" => serde_json::from_value(value).unwrap_or_else(|e2| {
                                    FinalPayload::Malformed {
                                        raw: trimmed.to_string(),
                                        error: format!("AstPayload parse error: {}", e2),
                                    }
                                }),
                                "semantic" => serde_json::from_value(value).unwrap_or_else(|e2| {
                                    FinalPayload::Malformed {
                                        raw: trimmed.to_string(),
                                        error: format!("SemanticPayload parse error: {}", e2),
                                    }
                                }),
                                _ => FinalPayload::Malformed {
                                    raw: trimmed.to_string(),
                                    error: format!("Unknown kind: {}", kind),
                                },
                            }
                        } else {
                            FinalPayload::Malformed {
                                raw: trimmed.to_string(),
                                error: format!("Missing 'kind' field: {}", e),
                            }
                        }
                    }
                }
            }
            Err(e) => {
                // Not valid JSON at all
                FinalPayload::Malformed {
                    raw: trimmed.to_string(),
                    error: format!("JSON parse error: {}", e),
                }
            }
        }
    }

    /// Check if this payload is verifiable by an oracle.
    pub fn is_verifiable(&self) -> bool {
        matches!(self, FinalPayload::Grep(_) | FinalPayload::Ast(_))
    }

    /// Get the file path this payload references (if any).
    pub fn file(&self) -> Option<&str> {
        match self {
            FinalPayload::Grep(p) => Some(&p.file),
            FinalPayload::Ast(p) => Some(&p.file),
            FinalPayload::Semantic(p) => Some(&p.file),
            FinalPayload::Malformed { .. } => None,
        }
    }

    /// Convert to a debuggable string representation.
    pub fn summary(&self) -> String {
        match self {
            FinalPayload::Grep(p) => {
                format!(
                    "Grep(file={}, pattern={}, {} matches)",
                    p.file,
                    p.pattern,
                    p.matches.len()
                )
            }
            FinalPayload::Ast(p) => {
                format!(
                    "Ast(file={}, query={}, {} results)",
                    p.file,
                    p.query,
                    p.results.len()
                )
            }
            FinalPayload::Semantic(p) => {
                let preview = if p.answer.len() > 50 {
                    format!("{}...", &p.answer[..50])
                } else {
                    p.answer.clone()
                };
                format!("Semantic(file={}, answer={})", p.file, preview)
            }
            FinalPayload::Malformed { error, .. } => {
                format!("Malformed({})", error)
            }
        }
    }
}

impl fmt::Display for FinalPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Grep/pattern-match payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GrepPayload {
    /// File that was searched
    pub file: String,
    /// Regex pattern used
    pub pattern: String,
    /// Matched lines
    pub matches: Vec<GrepMatch>,
}

/// A single grep match with line number and text.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GrepMatch {
    /// Line number (1-indexed, matching `grep -n`)
    pub line: usize,
    /// Full text of the matched line (or substring)
    pub text: String,
}

impl GrepMatch {
    /// Create a new match.
    pub fn new(line: usize, text: String) -> Self {
        Self { line, text }
    }

    /// Check if this match's text is a substring of the actual line.
    pub fn text_matches(&self, actual_line: &str) -> bool {
        actual_line.contains(&self.text)
    }
}

/// AST/structural query payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstPayload {
    /// File that was queried
    pub file: String,
    /// Tree-sitter query or query type
    pub query: String,
    /// Query results
    pub results: Vec<AstResult>,
}

/// A single AST query result.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstResult {
    /// Name of the matched item (function name, struct name, etc.)
    pub name: String,
    /// Function arguments/parameters (as string)
    #[serde(default)]
    pub args: Vec<String>,
    /// Return type (as string)
    #[serde(default)]
    pub return_type: Option<String>,
    /// Span: (start_line, end_line)
    #[serde(default)]
    pub span: Option<(usize, usize)>,
}

impl AstResult {
    /// Create a new AST result for a function.
    pub fn function(
        name: String,
        args: Vec<String>,
        return_type: Option<String>,
        span: Option<(usize, usize)>,
    ) -> Self {
        Self {
            name,
            args,
            return_type,
            span,
        }
    }
}

/// Semantic/free-form text payload (unverifiable).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SemanticPayload {
    /// File that was analyzed
    pub file: String,
    /// Free-form text answer
    pub answer: String,
}

impl SemanticPayload {
    /// Create a new semantic payload.
    pub fn new(file: String, answer: String) -> Self {
        Self { file, answer }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_grep_payload() {
        let json = r#"{
            "kind": "grep",
            "file": "src/main.rs",
            "pattern": "async fn",
            "matches": [
                {"line": 42, "text": "async fn process() {"},
                {"line": 100, "text": "async fn handle() {"}
            ]
        }"#;

        let payload = FinalPayload::parse(json);
        match payload {
            FinalPayload::Grep(p) => {
                assert_eq!(p.file, "src/main.rs");
                assert_eq!(p.pattern, "async fn");
                assert_eq!(p.matches.len(), 2);
                assert_eq!(p.matches[0].line, 42);
            }
            _ => panic!("Expected Grep payload"),
        }
    }

    #[test]
    fn parse_ast_payload() {
        let json = r#"{
            "kind": "ast",
            "file": "src/main.rs",
            "query": "functions",
            "results": [
                {"name": "process", "args": ["input: &str"], "return_type": "Result<String>"}
            ]
        }"#;

        let payload = FinalPayload::parse(json);
        match payload {
            FinalPayload::Ast(p) => {
                assert_eq!(p.file, "src/main.rs");
                assert_eq!(p.query, "functions");
                assert_eq!(p.results.len(), 1);
                assert_eq!(p.results[0].name, "process");
            }
            _ => panic!("Expected Ast payload"),
        }
    }

    #[test]
    fn parse_semantic_payload() {
        let json = r#"{
            "kind": "semantic",
            "file": "src/main.rs",
            "answer": "This module provides async processing."
        }"#;

        let payload = FinalPayload::parse(json);
        match payload {
            FinalPayload::Semantic(p) => {
                assert_eq!(p.file, "src/main.rs");
                assert!(p.answer.contains("async processing"));
            }
            _ => panic!("Expected Semantic payload"),
        }
    }

    #[test]
    fn parse_malformed_json() {
        let json = "not valid json at all";
        let payload = FinalPayload::parse(json);
        match payload {
            FinalPayload::Malformed { raw, error } => {
                assert_eq!(raw, "not valid json at all");
                assert!(error.contains("JSON parse error"));
            }
            _ => panic!("Expected Malformed payload"),
        }
    }

    #[test]
    fn parse_missing_kind_field() {
        let json = r#"{"file": "src/main.rs", "data": "value"}"#;
        let payload = FinalPayload::parse(json);
        match payload {
            FinalPayload::Malformed { error, .. } => {
                assert!(error.contains("kind"));
            }
            _ => panic!("Expected Malformed payload"),
        }
    }

    #[test]
    fn malformed_payload_is_serializable() {
        let payload = FinalPayload::Malformed {
            raw: "oops".to_string(),
            error: "parse error".to_string(),
        };
        let json = serde_json::to_string(&payload).expect("malformed payload should serialize");
        assert!(json.contains("\"kind\":\"malformed\""));
        assert!(json.contains("\"raw\":\"oops\""));
        assert!(json.contains("\"error\":\"parse error\""));
    }

    #[test]
    fn grep_match_text_matching() {
        let m = GrepMatch::new(42, "async fn".to_string());
        assert!(m.text_matches("pub async fn process() -> Result<()> {"));
        assert!(!m.text_matches("fn process() -> Result<()> {"));
    }

    #[test]
    fn is_verifiable() {
        let grep_json = r#"{"kind": "grep", "file": "x.rs", "pattern": "fn", "matches": []}"#;
        let semantic_json = r#"{"kind": "semantic", "file": "x.rs", "answer": "text"}"#;

        assert!(FinalPayload::parse(grep_json).is_verifiable());
        assert!(!FinalPayload::parse(semantic_json).is_verifiable());
    }

    #[test]
    fn file_extraction() {
        let grep_json =
            r#"{"kind": "grep", "file": "src/main.rs", "pattern": "fn", "matches": []}"#;
        assert_eq!(FinalPayload::parse(grep_json).file(), Some("src/main.rs"));
    }
}
