//! Deterministic oracle system for validating RLM REPL trace outputs.
//!
//! This module provides oracles that can verify FINAL() answers from RLM traces
//! without requiring cloud LLM judges. This enables synthetic training data
//! generation for the BitNet distilled navigation model.
//!
//! # Architecture
//!
//! - **Grep Oracle**: Pattern-match verification (e.g., "find all async functions")
//! - **Tree-sitter Oracle**: Structural AST verification (function signatures, etc.)
//! - **Validator Pipeline**: Routes queries to appropriate oracles and outputs golden traces
//!
//! # Usage
//!
//! ```ignore
//! use codetether_agent::rlm::oracle::{TraceValidator, OracleResult};
//!
//! let validator = TraceValidator::new();
//! let result = validator.validate(&analysis_result, &source_file).await;
//!
//! match result {
//!     OracleResult::Golden(trace) => save_to_jsonl(trace),
//!     OracleResult::Unverified => {} // No oracle available
//!     OracleResult::Failed(reason) => {} // Oracle disagrees
//! }
//! ```

mod grep_oracle;
mod schema;
mod templates;
mod tree_sitter_oracle;
mod validator;

pub use grep_oracle::GrepOracle;
pub use schema::{AstPayload, AstResult, FinalPayload, GrepMatch, GrepPayload, SemanticPayload};
pub use templates::{GeneratedQuery, QueryTemplate, TemplateKind};
pub use tree_sitter_oracle::TreeSitterOracle;
pub use validator::{OracleResult, TraceValidator, ValidatedTrace, VerificationMethod};

/// Query type classification for routing to the appropriate oracle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryType {
    /// Pattern-match query (grep-based: line numbers, text matches)
    PatternMatch,
    /// Structural query (AST-based: function signatures, struct fields)
    Structural,
    /// Semantic query (requires LLM understanding - no deterministic oracle)
    Semantic,
}

/// Classification of an RLM FINAL() answer format.
#[derive(Debug, Clone, PartialEq)]
pub enum FinalAnswerFormat {
    /// Line-numbered matches (e.g., "42:async fn foo()", "100:pub struct Bar")
    LineNumberedMatches {
        matches: Vec<(usize, String)>,
    },
    /// Count result (e.g., "Found 15 occurrences")
    CountResult {
        count: usize,
    },
    /// Structured data (e.g., function signature JSON)
    StructuredData {
        data: serde_json::Value,
    },
    /// Free-form text (semantic - no deterministic verification)
    FreeFormText {
        text: String,
    },
}

impl FinalAnswerFormat {
    /// Parse a FINAL() answer string into its classified format.
    pub fn parse(answer: &str) -> Self {
        // Try to parse as line-numbered matches
        let lines: Vec<&str> = answer.lines().collect();
        let mut numbered_matches = Vec::new();
        let mut all_valid = true;

        for line in &lines {
            // Pattern: "42:text" or "42: text" or "L42: text"
            let trimmed = line.trim();
            if let Some(colon_pos) = trimmed.find(':') {
                let num_part = trimmed[..colon_pos]
                    .trim()
                    .trim_start_matches('L')
                    .trim();
                if let Ok(line_num) = num_part.parse::<usize>() {
                    let text_part = trimmed[colon_pos + 1..].trim().to_string();
                    numbered_matches.push((line_num, text_part));
                } else {
                    all_valid = false;
                    break;
                }
            } else if !trimmed.is_empty() {
                // Non-empty line without line number
                all_valid = false;
                break;
            }
        }

        if all_valid && !numbered_matches.is_empty() {
            return Self::LineNumberedMatches {
                matches: numbered_matches,
            };
        }

        // Try to parse as count result
        let lower = answer.to_lowercase();
        if lower.contains("found") || lower.contains("count:") || lower.contains("occurrences") {
            // Extract number from text like "Found 15 async functions"
            if let Some(count) = extract_count_from_text(answer) {
                return Self::CountResult { count };
            }
        }

        // Try to parse as JSON
        if answer.trim().starts_with('{') || answer.trim().starts_with('[') {
            if let Ok(data) = serde_json::from_str::<serde_json::Value>(answer) {
                return Self::StructuredData { data };
            }
        }

        // Default to free-form text
        Self::FreeFormText {
            text: answer.to_string(),
        }
    }
}

/// Extract a count number from natural language text.
fn extract_count_from_text(text: &str) -> Option<usize> {
    // Look for patterns like "15 functions", "count: 42", "Found 7"
    let re = regex::Regex::new(r"(?i)(?:found|count:?\s*)\s*(\d+)|(\d+)\s+(?:functions?|matches?|occurrences?|items?|results?)").ok()?;
    
    for cap in re.captures_iter(text) {
        // Try first group (found/count)
        if let Some(m) = cap.get(1) {
            if let Ok(n) = m.as_str().parse() {
                return Some(n);
            }
        }
        // Try second group (number before word)
        if let Some(m) = cap.get(2) {
            if let Ok(n) = m.as_str().parse() {
                return Some(n);
            }
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_numbered_matches() {
        let answer = "42:async fn foo()\n100:pub struct Bar\n";
        let format = FinalAnswerFormat::parse(answer);
        match format {
            FinalAnswerFormat::LineNumberedMatches { matches } => {
                assert_eq!(matches.len(), 2);
                assert_eq!(matches[0], (42, "async fn foo()".to_string()));
                assert_eq!(matches[1], (100, "pub struct Bar".to_string()));
            }
            _ => panic!("Expected LineNumberedMatches"),
        }
    }

    #[test]
    fn parse_count_result() {
        let answer = "Found 15 async functions";
        let format = FinalAnswerFormat::parse(answer);
        match format {
            FinalAnswerFormat::CountResult { count } => assert_eq!(count, 15),
            _ => panic!("Expected CountResult"),
        }
    }

    #[test]
    fn parse_structured_data() {
        let answer = r#"{"name": "foo", "args": ["x", "y"]}"#;
        let format = FinalAnswerFormat::parse(answer);
        match format {
            FinalAnswerFormat::StructuredData { data } => {
                assert_eq!(data["name"], "foo");
            }
            _ => panic!("Expected StructuredData"),
        }
    }

    #[test]
    fn parse_free_form_text() {
        let answer = "This function handles error cases by using the ? operator";
        let format = FinalAnswerFormat::parse(answer);
        match format {
            FinalAnswerFormat::FreeFormText { text } => {
                assert!(text.contains("error cases"));
            }
            _ => panic!("Expected FreeFormText"),
        }
    }
}
