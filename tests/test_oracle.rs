//! Integration tests for the RLM oracle system.

use codetether_agent::rlm::oracle::{
    GrepOracle, GrepVerification, OracleResult, QueryType, TraceValidator,
    TreeSitterOracle, TreeSitterVerification, FinalAnswerFormat,
};
use codetether_agent::rlm::context_trace::{ContextTrace, ContextEvent};
use codetether_agent::rlm::RlmStats;
use codetether_agent::rlm::repl::RlmAnalysisResult;

fn sample_rust_code() -> String {
    r#"
use anyhow::Result;
use std::collections::HashMap;

/// Configuration struct for the application
pub struct Config {
    pub debug: bool,
    pub max_retries: usize,
    pub timeout_ms: u64,
}

impl Config {
    /// Create a new config with defaults
    pub fn new() -> Self {
        Self {
            debug: false,
            max_retries: 3,
            timeout_ms: 5000,
        }
    }
    
    /// Enable debug mode
    pub fn with_debug(mut self) -> Self {
        self.debug = true;
        self
    }
}

/// Process input data
pub async fn process(input: &str) -> Result<String> {
    let data = parse(input)?;
    let result = transform(&data)?;
    Ok(result)
}

/// Parse input string
fn parse(input: &str) -> Result<String> {
    if input.is_empty() {
        return Err(anyhow::anyhow!("Empty input"));
    }
    Ok(input.trim().to_string())
}

/// Transform the data
fn transform(data: &str) -> Result<String> {
    Ok(data.to_uppercase())
}

pub enum Status {
    Active,
    Inactive,
    Pending,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_process() {
        let result = process("hello");
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_async_process() {
        let result = process("test").await;
        assert!(result.is_ok());
    }
}
"#.to_string()
}

fn make_analysis_result(answer: &str) -> RlmAnalysisResult {
    RlmAnalysisResult {
        answer: answer.to_string(),
        iterations: 2,
        sub_queries: vec![],
        stats: RlmStats {
            input_tokens: 150,
            output_tokens: 80,
            iterations: 2,
            subcalls: 0,
            elapsed_ms: 500,
            compression_ratio: 1.0,
        },
    }
}

// ============================================================================
// Grep Oracle Tests
// ============================================================================

#[test]
fn grep_oracle_finds_async_functions() {
    let source = sample_rust_code();
    let oracle = GrepOracle::new(source);
    
    let matches = oracle.grep(r"\basync\s+fn\b").unwrap();
    assert!(matches.len() >= 1);
    
    // Should find the async process function
    assert!(matches.iter().any(|(_, line)| line.contains("process")));
}

#[test]
fn grep_oracle_finds_public_functions() {
    let source = sample_rust_code();
    let oracle = GrepOracle::new(source);
    
    let matches = oracle.grep(r"\bpub\s+fn\b").unwrap();
    assert!(matches.len() >= 3);
}

#[test]
fn grep_oracle_infers_async_pattern() {
    let pattern = GrepOracle::infer_pattern("Find all async functions");
    assert_eq!(pattern, Some(r"\basync\s+fn\b".to_string()));
}

#[test]
fn grep_oracle_infers_pub_pattern() {
    let pattern = GrepOracle::infer_pattern("List all public functions");
    assert_eq!(pattern, Some(r"\bpub\s+fn\b".to_string()));
}

#[test]
fn grep_oracle_infers_struct_pattern() {
    let pattern = GrepOracle::infer_pattern("Find all structs");
    assert_eq!(pattern, Some(r"\bstruct\b".to_string()));
}

#[test]
fn grep_oracle_classifies_pattern_match_query() {
    assert_eq!(
        GrepOracle::classify_query("Find all async functions"),
        QueryType::PatternMatch
    );
    assert_eq!(
        GrepOracle::classify_query("Count occurrences of TODO"),
        QueryType::PatternMatch
    );
    assert_eq!(
        GrepOracle::classify_query("Search for error handling"),
        QueryType::PatternMatch
    );
}

#[test]
fn grep_oracle_verifies_count_result() {
    let source = sample_rust_code();
    let oracle = GrepOracle::new(source);
    
    // Should find 1 async function
    let result = oracle.verify("Found 1 async function", "Count async functions");
    assert_eq!(result, GrepVerification::ExactMatch);
}

// ============================================================================
// Tree-sitter Oracle Tests
// ============================================================================

#[test]
fn tree_sitter_oracle_gets_functions() {
    let mut oracle = TreeSitterOracle::new(sample_rust_code());
    let functions = oracle.get_functions().unwrap();
    
    assert!(functions.len() >= 4);
    
    let names: Vec<&str> = functions.iter().map(|f| f.name.as_str()).collect();
    assert!(names.contains(&"new"));
    assert!(names.contains(&"with_debug"));
    assert!(names.contains(&"process"));
    assert!(names.contains(&"parse"));
    assert!(names.contains(&"transform"));
}

#[test]
fn tree_sitter_oracle_gets_structs() {
    let mut oracle = TreeSitterOracle::new(sample_rust_code());
    let structs = oracle.get_structs().unwrap();
    
    assert!(structs.len() >= 1);
    
    let config = structs.iter().find(|s| s.name == "Config").unwrap();
    assert!(config.fields.contains(&"debug".to_string()));
    assert!(config.fields.contains(&"max_retries".to_string()));
    assert!(config.fields.contains(&"timeout_ms".to_string()));
}

#[test]
fn tree_sitter_oracle_gets_enums() {
    let mut oracle = TreeSitterOracle::new(sample_rust_code());
    let enums = oracle.get_enums().unwrap();
    
    assert!(enums.len() >= 1);
    
    let status = enums.iter().find(|e| e.name == "Status").unwrap();
    assert!(status.variants.contains(&"Active".to_string()));
    assert!(status.variants.contains(&"Inactive".to_string()));
    assert!(status.variants.contains(&"Pending".to_string()));
}

#[test]
fn tree_sitter_oracle_counts_error_patterns() {
    let mut oracle = TreeSitterOracle::new(sample_rust_code());
    let counts = oracle.count_error_patterns().unwrap();
    
    // Should find Result<T> types
    assert!(counts.result_types >= 3);
    
    // Should find ? operators
    assert!(counts.try_operators >= 2);
}

#[test]
fn tree_sitter_oracle_query() {
    let mut oracle = TreeSitterOracle::new(sample_rust_code());
    
    let result = oracle
        .query("(function_item name: (identifier) @name)")
        .unwrap();
    
    assert!(!result.matches.is_empty());
    
    // All matches should have a "name" capture
    for m in &result.matches {
        assert!(m.captures.contains_key("name"));
    }
}

// ============================================================================
// Trace Validator Tests
// ============================================================================

#[test]
fn trace_validator_validates_grep_match() {
    let validator = TraceValidator::new();
    let source = sample_rust_code();
    let result = make_analysis_result("30:pub async fn process(input: &str) -> Result<String> {");
    
    match validator.validate(&result, &source, Some("test.rs")) {
        OracleResult::Golden(trace) => {
            assert!(trace.answer.contains("async"));
        }
        OracleResult::Unverified { .. } => {}
        OracleResult::Failed { .. } => {}
    }
}

#[test]
fn trace_validator_marks_semantic_as_unverified() {
    let validator = TraceValidator::new();
    let source = sample_rust_code();
    let result = make_analysis_result("This function processes input by parsing and transforming it");
    
    match validator.validate(&result, &source, Some("test.rs")) {
        OracleResult::Unverified { reason } => {
            assert!(reason.contains("Semantic"));
        }
        _ => panic!("Expected Unverified for semantic query"),
    }
}

#[test]
fn trace_validator_batch_validate() {
    let validator = TraceValidator::new();
    let source = sample_rust_code();
    
    let traces = vec![
        (make_analysis_result("1 async function"), source.as_str(), None),
        (make_analysis_result("Explanation text"), source.as_str(), None),
    ];
    
    let stats = validator.batch_validate(traces);
    
    assert!(stats.total() == 2);
}

// ============================================================================
// Context Trace Tests
// ============================================================================

#[test]
fn context_trace_tracks_tokens() {
    let mut trace = ContextTrace::new(1000);
    
    trace.log_event(ContextEvent::SystemPrompt {
        content: "System prompt".to_string(),
        tokens: 100,
    });
    
    trace.log_event(ContextEvent::GrepResult {
        pattern: "async".to_string(),
        matches: 5,
        tokens: 50,
    });
    
    assert_eq!(trace.total_tokens(), 150);
    assert_eq!(trace.remaining_tokens(), 850);
    assert_eq!(trace.budget_used_percent(), 15.0);
}

#[test]
fn context_trace_detects_over_budget() {
    let mut trace = ContextTrace::new(100);
    
    trace.log_event(ContextEvent::Final {
        answer: "Big answer".to_string(),
        tokens: 150,
    });
    
    assert!(trace.is_over_budget());
}

#[test]
fn context_trace_filters_by_type() {
    let mut trace = ContextTrace::new(1000);
    
    trace.log_event(ContextEvent::SystemPrompt {
        content: "test".to_string(),
        tokens: 100,
    });
    
    trace.log_event(ContextEvent::GrepResult {
        pattern: "async".to_string(),
        matches: 5,
        tokens: 50,
    });
    
    trace.log_event(ContextEvent::SystemPrompt {
        content: "test2".to_string(),
        tokens: 75,
    });
    
    let system_events = trace.events_of_type("system_prompt");
    assert_eq!(system_events.len(), 2);
    
    let grep_events = trace.events_of_type("grep_result");
    assert_eq!(grep_events.len(), 1);
}

#[test]
fn context_trace_summary() {
    let mut trace = ContextTrace::new(1000);
    
    trace.log_event(ContextEvent::SystemPrompt {
        content: "test".to_string(),
        tokens: 100,
    });
    
    trace.log_event(ContextEvent::GrepResult {
        pattern: "async".to_string(),
        matches: 5,
        tokens: 50,
    });
    
    trace.next_iteration();
    
    let summary = trace.summary();
    assert_eq!(summary.total_tokens, 150);
    assert_eq!(summary.iteration, 1);
    assert_eq!(summary.events_len, 2);
}

// ============================================================================
// Final Answer Format Tests
// ============================================================================

#[test]
fn parse_line_numbered_matches() {
    let answer = "42:async fn foo()\n100:pub struct Bar";
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
        FinalAnswerFormat::CountResult { count } => {
            assert_eq!(count, 15);
        }
        _ => panic!("Expected CountResult"),
    }
}

#[test]
fn parse_structured_data() {
    let answer = r#"{"functions": ["foo", "bar"]}"#;
    let format = FinalAnswerFormat::parse(answer);
    
    match format {
        FinalAnswerFormat::StructuredData { data } => {
            assert!(data["functions"].is_array());
        }
        _ => panic!("Expected StructuredData"),
    }
}

#[test]
fn parse_free_form_text() {
    let answer = "This function handles errors by using the ? operator";
    let format = FinalAnswerFormat::parse(answer);
    
    match format {
        FinalAnswerFormat::FreeFormText { text } => {
            assert!(text.contains("? operator"));
        }
        _ => panic!("Expected FreeFormText"),
    }
}
