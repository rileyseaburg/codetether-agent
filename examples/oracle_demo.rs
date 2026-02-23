//! Example usage of the RLM oracle system.
//!
//! Run with: cargo run --example oracle_demo

use codetether_agent::rlm::oracle::{
    GrepOracle, OracleResult, TraceValidator, TreeSitterOracle,
};
use codetether_agent::rlm::context_trace::{ContextTrace, ContextEvent};

fn main() {
    println!("=== RLM Oracle System Demo ===\n");
    
    // Sample Rust source code
    let source_code = r#"
use anyhow::Result;

pub struct Config {
    pub debug: bool,
    pub timeout: u64,
}

impl Config {
    pub fn new() -> Self {
        Self { debug: false, timeout: 5000 }
    }
}

pub async fn process(input: &str) -> Result<String> {
    let data = parse(input)?;
    Ok(data.to_uppercase())
}

fn parse(input: &str) -> Result<String> {
    Ok(input.trim().to_string())
}

enum Status {
    Active,
    Inactive,
}
"#;
    
    // =========================================================================
    // Grep Oracle Demo
    // =========================================================================
    println!("1. Grep Oracle");
    println!("--------------");
    
    let grep_oracle = GrepOracle::new(source_code.to_string());
    
    // Find async functions
    let matches = grep_oracle.grep(r"\basync\s+fn\b").unwrap();
    println!("Async functions found:");
    for (line, text) in &matches {
        println!("  L{}: {}", line, text.trim());
    }
    println!();
    
    // Verify a FINAL() answer
    let answer = "21:pub async fn process(input: &str) -> Result<String> {";
    let verification = grep_oracle.verify(answer, "Find all async functions");
    println!("Verification result: {:?}", verification);
    println!();
    
    // =========================================================================
    // Tree-sitter Oracle Demo
    // =========================================================================
    println!("2. Tree-sitter Oracle");
    println!("---------------------");
    
    let mut ts_oracle = TreeSitterOracle::new(source_code.to_string());
    
    // Get all functions
    println!("Functions:");
    match ts_oracle.get_functions() {
        Ok(functions) => {
            for func in functions {
                println!("  L{}: {}({})", func.line, func.name, func.params);
            }
        }
        Err(e) => println!("  Error: {}", e),
    }
    println!();
    
    // Get all structs
    println!("Structs:");
    match ts_oracle.get_structs() {
        Ok(structs) => {
            for s in structs {
                println!("  L{}: {} {{ {} }}", s.line, s.name, s.fields.join(", "));
            }
        }
        Err(e) => println!("  Error: {}", e),
    }
    println!();
    
    // Count error patterns
    println!("Error patterns:");
    match ts_oracle.count_error_patterns() {
        Ok(counts) => {
            println!("  Result<T> types: {}", counts.result_types);
            println!("  ? operators: {}", counts.try_operators);
        }
        Err(e) => println!("  Error: {}", e),
    }
    println!();
    
    // =========================================================================
    // Context Trace Demo
    // =========================================================================
    println!("3. Context Trace");
    println!("----------------");
    
    let mut trace = ContextTrace::new(4000);
    
    trace.log_event(ContextEvent::SystemPrompt {
        content: "You are a code analysis assistant.".to_string(),
        tokens: 50,
    });
    
    trace.log_event(ContextEvent::GrepResult {
        pattern: "async fn".to_string(),
        matches: 1,
        tokens: 100,
    });
    
    trace.log_event(ContextEvent::Final {
        answer: "Found 1 async function".to_string(),
        tokens: 20,
    });
    
    let summary = trace.summary();
    println!("{}", summary.format());
    println!();
    
    // =========================================================================
    // Trace Validator Demo
    // =========================================================================
    println!("4. Trace Validator");
    println!("------------------");
    
    println!("Validator can verify pattern-match and structural queries,");
    println!("producing golden traces for training data generation.");
    println!();
    println!("Query types supported:");
    println!("  - Pattern-match: grep-based (find, list, count, search)");
    println!("  - Structural: AST-based (signatures, fields, impls)");
    println!("  - Semantic: unverified (explanations, why, how)");
    println!();
    
    // =========================================================================
    // Training Data Generation
    // =========================================================================
    println!("5. Training Data Output");
    println!("-----------------------");
    println!("Golden traces are output as JSONL for SFT training:");
    println!();
    println!(r#"{{
  "query": "Find all async functions",
  "answer": "21:async fn process()",
  "iterations": 2,
  "input_tokens": 150,
  "output_tokens": 80,
  "verification_method": "GrepOracle"
}}"#);
    println!();
    
    println!("=== Demo Complete ===");
}
