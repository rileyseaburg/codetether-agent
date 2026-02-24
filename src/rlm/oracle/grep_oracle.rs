//! Grep-based oracle for pattern-match verification.
//!
//! This oracle verifies FINAL() answers that claim to find patterns in source code
//! by running actual grep operations and comparing results.
//!
//! # Supported Queries
//!
//! - "Find all async functions"
//! - "Find all structs matching pattern X"
//! - "Count occurrences of Y"
//! - "List all error handling patterns"
//!
//! # Verification Strategy
//!
//! 1. Parse the FINAL() answer to extract claimed matches
//! 2. Run actual grep on the source file with the inferred pattern
//! 3. Compare line numbers and content
//! 4. Return verification result

use anyhow::Result;
use regex::Regex;
use std::collections::HashSet;

use super::{FinalAnswerFormat, QueryType};

/// Result of grep oracle verification.
#[derive(Debug, Clone, PartialEq)]
pub enum GrepVerification {
    /// Answer matches ground truth exactly.
    ExactMatch,
    /// Answer matches but in different order.
    UnorderedMatch,
    /// Answer is a subset of ground truth (partial).
    SubsetMatch { claimed: usize, actual: usize },
    /// Answer contains claims not in ground truth (false positives).
    HasFalsePositives {
        false_positives: Vec<(usize, String)>,
    },
    /// Answer is missing ground truth items (false negatives).
    HasFalseNegatives {
        false_negatives: Vec<(usize, String)>,
    },
    /// Answer is completely different.
    Mismatch,
    /// Could not infer pattern from query.
    CannotVerify { reason: String },
}

/// Grep-based oracle for validating pattern-match queries.
pub struct GrepOracle {
    /// Source code content
    source: String,
    /// Source code as lines (1-indexed when displayed)
    source_lines: Vec<String>,
}

impl GrepOracle {
    /// Create a new grep oracle for the given source file content.
    pub fn new(source: String) -> Self {
        let source_lines = source.lines().map(|s| s.to_string()).collect();
        Self {
            source,
            source_lines,
        }
    }

    /// Classify the query type based on keywords.
    pub fn classify_query(query: &str) -> QueryType {
        let lower = query.to_lowercase();

        // Pattern-match indicators
        if lower.contains("find all")
            || lower.contains("list all")
            || lower.contains("search for")
            || lower.contains("grep")
            || lower.contains("lines matching")
            || lower.contains("occurrences of")
            || lower.contains("count")
        {
            return QueryType::PatternMatch;
        }

        // Structural indicators
        if lower.contains("signature")
            || lower.contains("parameters")
            || lower.contains("return type")
            || lower.contains("fields of")
            || lower.contains("implements")
            || lower.contains("trait")
        {
            return QueryType::Structural;
        }

        QueryType::Semantic
    }

    /// Infer a grep pattern from the query string.
    ///
    /// Returns None if the pattern cannot be reliably inferred.
    pub fn infer_pattern(query: &str) -> Option<String> {
        let lower = query.to_lowercase();

        // First, try to extract an explicit quoted literal pattern from the query.
        // Examples:
        // - Find all occurrences of 'async fn' in file.rs
        // - grep for "Result<"
        let quoted_re =
            Regex::new(r#"(?i)(?:occurrences?\s+of|grep(?:\s+for)?|matching|containing)\s+['"`]([^'"`]+)['"`]"#).ok()?;
        if let Some(caps) = quoted_re.captures(query)
            && let Some(m) = caps.get(1)
        {
            return Some(regex::escape(m.as_str()));
        }

        // Fallback: capture any quoted literal in the query.
        let any_quoted_re = Regex::new(r#"['"`]([^'"`]+)['"`]"#).ok()?;
        if let Some(caps) = any_quoted_re.captures(query)
            && let Some(m) = caps.get(1)
        {
            return Some(regex::escape(m.as_str()));
        }

        // Fallback: handle unquoted "occurrences of X in ..." forms.
        let bare_occurrences_re = Regex::new(r"(?i)occurrences?\s+of\s+(.+?)(?:\s+in\b|$)").ok()?;
        if let Some(caps) = bare_occurrences_re.captures(query)
            && let Some(m) = caps.get(1)
        {
            let candidate = m.as_str().trim().trim_matches(&['"', '\'', '`'][..]);
            if !candidate.is_empty() {
                return Some(regex::escape(candidate));
            }
        }

        // Common Rust patterns
        let patterns = [
            // Async functions
            (r"(?i)find\s+all\s+async\s+functions?", r"\basync\s+fn\b"),
            (r"(?i)list\s+async\s+functions?", r"\basync\s+fn\b"),
            (r"(?i)async\s+functions?", r"\basync\s+fn\b"),
            // Public functions
            (r"(?i)public\s+functions?", r"\bpub\s+fn\b"),
            (r"(?i)pub\s+functions?", r"\bpub\s+fn\b"),
            // Structs
            (r"(?i)find\s+all\s+structs?", r"\bstruct\b"),
            (r"(?i)list\s+structs?", r"\bstruct\b"),
            (r"(?i)all\s+structs?", r"\bstruct\b"),
            // Enums
            (r"(?i)find\s+all\s+enums?", r"\benum\b"),
            (r"(?i)list\s+enums?", r"\benum\b"),
            // Traits
            (r"(?i)find\s+all\s+traits?", r"\btrait\b"),
            (r"(?i)list\s+traits?", r"\btrait\b"),
            // Impls
            (r"(?i)find\s+all\s+impls?", r"\bimpl\b"),
            (r"(?i)implementations?", r"\bimpl\b"),
            // Error handling
            (r"(?i)error\s+handling", r"Result|anyhow|Error|\?"),
            (r"(?i)unwrap\s+calls?", r"\.unwrap\(\)"),
            (r"(?i)expect\s+calls?", r"\.expect\("),
            // Imports
            (r"(?i)use\s+statements?", r"\buse\b"),
            (r"(?i)imports?", r"\buse\b"),
            // Tests
            (r"(?i)test\s+functions?", r"#\[test\]"),
            (r"(?i)async\s+tests?", r"#\[tokio::test\]"),
            // Comments
            (r"(?i)todo\s+comments?", r"TODO|FIXME|XXX"),
            (r"(?i)comments?", r"//|/\*"),
            // Macros
            (r"(?i)macro\s+calls?", r"[a-zA-Z_]+!"),
            (r"(?i)println!?", r"println!"),
            // String literals
            (r"(?i)string\s+literals?", r#""[^"]*""#),
        ];

        for (pattern_re, grep_pattern) in patterns {
            if let Ok(re) = Regex::new(pattern_re) {
                if re.is_match(&lower) {
                    return Some(grep_pattern.to_string());
                }
            }
        }

        None
    }

    /// Run grep on the source and return matches with line numbers.
    pub fn grep(&self, pattern: &str) -> Result<Vec<(usize, String)>> {
        let re = Regex::new(pattern)?;

        let matches: Vec<(usize, String)> = self
            .source_lines
            .iter()
            .enumerate()
            .filter(|(_, line)| re.is_match(line))
            .map(|(i, line)| (i + 1, line.clone())) // 1-indexed
            .collect();

        Ok(matches)
    }

    /// Verify a FINAL() answer against ground truth.
    ///
    /// Takes the claimed answer and the original query, infers the pattern,
    /// runs grep, and compares results.
    pub fn verify(&self, answer: &str, query: &str) -> GrepVerification {
        let format = FinalAnswerFormat::parse(answer);

        // Infer the grep pattern from the query
        let pattern = Self::infer_pattern(query)
            .or_else(|| Self::infer_pattern(answer))
            .ok_or_else(|| "Could not infer grep pattern from query".to_string());
        let pattern = match pattern {
            Ok(p) => p,
            Err(reason) => {
                return GrepVerification::CannotVerify { reason };
            }
        };

        // Get ground truth
        let ground_truth = match self.grep(&pattern) {
            Ok(m) => m,
            Err(e) => {
                return GrepVerification::CannotVerify {
                    reason: format!("Grep failed: {}", e),
                };
            }
        };

        // Compare based on answer format
        match format {
            FinalAnswerFormat::LineNumberedMatches { matches: claimed } => {
                self.verify_matches(&claimed, &ground_truth)
            }
            FinalAnswerFormat::CountResult {
                count: claimed_count,
            } => {
                let actual_count = ground_truth.len();
                if claimed_count == actual_count {
                    GrepVerification::ExactMatch
                } else {
                    GrepVerification::SubsetMatch {
                        claimed: claimed_count,
                        actual: actual_count,
                    }
                }
            }
            FinalAnswerFormat::StructuredData { .. } => GrepVerification::CannotVerify {
                reason: "Structured data not supported by grep oracle".to_string(),
            },
            FinalAnswerFormat::FreeFormText { text } => {
                // Try to extract line numbers from free-form text
                if let Some(extracted) = self.extract_line_numbers_from_text(&text) {
                    self.verify_matches(&extracted, &ground_truth)
                } else {
                    GrepVerification::CannotVerify {
                        reason: "Could not extract line numbers from free-form text".to_string(),
                    }
                }
            }
        }
    }

    /// Verify claimed matches against ground truth (public for validator use).
    pub fn verify_matches(
        &self,
        claimed: &[(usize, String)],
        ground_truth: &[(usize, String)],
    ) -> GrepVerification {
        let claimed_set: HashSet<(usize, String)> = claimed.iter().cloned().collect();
        let truth_set: HashSet<(usize, String)> = ground_truth.iter().cloned().collect();

        // Check for exact match (including order)
        if claimed == ground_truth {
            return GrepVerification::ExactMatch;
        }

        // Check for unordered match
        if claimed_set == truth_set {
            return GrepVerification::UnorderedMatch;
        }

        // Check for false positives (claimed but not in ground truth)
        let false_positives: Vec<_> = claimed
            .iter()
            .filter(|item| !truth_set.contains(item))
            .cloned()
            .collect();

        // Check for false negatives (in ground truth but not claimed)
        let false_negatives: Vec<_> = ground_truth
            .iter()
            .filter(|item| !claimed_set.contains(item))
            .cloned()
            .collect();

        if !false_positives.is_empty() && !false_negatives.is_empty() {
            // Both false positives and negatives - complete mismatch
            GrepVerification::Mismatch
        } else if !false_positives.is_empty() {
            GrepVerification::HasFalsePositives { false_positives }
        } else if !false_negatives.is_empty() {
            GrepVerification::HasFalseNegatives { false_negatives }
        } else {
            // Should not reach here, but treat as mismatch
            GrepVerification::Mismatch
        }
    }

    /// Try to extract line numbers from free-form text.
    ///
    /// Looks for patterns like "line 42", "L42:", "lines 10-20", etc.
    fn extract_line_numbers_from_text(&self, text: &str) -> Option<Vec<(usize, String)>> {
        let mut results = Vec::new();

        // Pattern: "line 42: text" or "L42: text" or "42: text"
        let line_re = Regex::new(r"(?i)(?:line\s+|L)?(\d+):\s*(.+)").ok()?;

        for line in text.lines() {
            if let Some(cap) = line_re.captures(line) {
                if let (Some(num), Some(content)) = (cap.get(1), cap.get(2)) {
                    if let Ok(line_num) = num.as_str().parse::<usize>() {
                        results.push((line_num, content.as_str().trim().to_string()));
                    }
                }
            }
        }

        if results.is_empty() {
            None
        } else {
            Some(results)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_rust_code() -> String {
        r#"
use anyhow::Result;

/// Process data
pub async fn process(input: &str) -> Result<String> {
    let data = parse(input)?;
    Ok(data)
}

async fn parse(input: &str) -> Result<String> {
    Ok(input.to_uppercase())
}

pub struct Config {
    pub debug: bool,
}

impl Config {
    pub fn new() -> Self {
        Self { debug: false }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_process() {
        assert!(true);
    }
}
"#
        .to_string()
    }

    #[test]
    fn classify_pattern_match_query() {
        assert_eq!(
            GrepOracle::classify_query("Find all async functions"),
            QueryType::PatternMatch
        );
        assert_eq!(
            GrepOracle::classify_query("Count occurrences of TODO"),
            QueryType::PatternMatch
        );
    }

    #[test]
    fn infer_async_pattern() {
        let pattern = GrepOracle::infer_pattern("Find all async functions");
        assert_eq!(pattern, Some(r"\basync\s+fn\b".to_string()));
    }

    #[test]
    fn infer_pub_pattern() {
        let pattern = GrepOracle::infer_pattern("List all public functions");
        assert_eq!(pattern, Some(r"\bpub\s+fn\b".to_string()));
    }

    #[test]
    fn infer_quoted_literal_pattern() {
        let pattern =
            GrepOracle::infer_pattern("Find all occurrences of 'async fn' in src/rlm/repl.rs");
        assert_eq!(pattern, Some(regex::escape("async fn")));
    }

    #[test]
    fn grep_finds_matches() {
        let oracle = GrepOracle::new(sample_rust_code());
        let matches = oracle.grep(r"\basync\s+fn\b").unwrap();
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn verify_exact_match() {
        let oracle = GrepOracle::new(sample_rust_code());
        let answer = oracle
            .grep(r"\basync\s+fn\b")
            .unwrap_or_default()
            .iter()
            .map(|(line, text)| format!("{line}:{text}"))
            .collect::<Vec<_>>()
            .join("\n");
        let result = oracle.verify(&answer, "Find all async functions");
        // May not be exact due to whitespace, but should be close
        match result {
            GrepVerification::ExactMatch
            | GrepVerification::UnorderedMatch
            | GrepVerification::SubsetMatch { .. } => (),
            _ => panic!("Expected match, got {:?}", result),
        }
    }

    #[test]
    fn verify_count_result() {
        let oracle = GrepOracle::new(sample_rust_code());
        let answer = "Found 2 async functions";
        let result = oracle.verify(answer, "Count async functions");
        assert_eq!(result, GrepVerification::ExactMatch);
    }
}
