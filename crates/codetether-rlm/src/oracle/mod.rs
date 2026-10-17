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

mod ast_validation;
mod batch;
mod batch_write;
mod consensus;
mod consensus_helpers;
mod grep_oracle;
mod grep_validation;
mod query_type;
mod record;
mod schema;
#[path = "storage/mod.rs"]
mod storage;
mod templates;
mod trace_types;
mod tree_sitter_oracle;
mod types;
#[path = "validator/mod.rs"]
mod validator;

pub use grep_oracle::{GrepOracle, GrepVerification};
pub use query_type::QueryType;
pub use record::OracleTraceRecord;
pub use schema::{AstPayload, AstResult, FinalPayload, GrepMatch, GrepPayload, SemanticPayload};
pub use storage::{
    OracleTracePersistResult, OracleTraceStorage, OracleTraceSyncStats, default_spool_dir,
};
pub use templates::{GeneratedQuery, QueryTemplate, TemplateKind};
pub use trace_types::{OracleResult, ValidatedTrace};
pub use tree_sitter_oracle::{AstCapture, TreeSitterOracle, TreeSitterVerification};
pub use types::{TraceStep, VerificationMethod};
pub use validator::{BatchValidationStats, SplitWriteStats, TraceValidator};

mod answer_format;
pub use answer_format::FinalAnswerFormat;
#[cfg(test)]
mod contract;
