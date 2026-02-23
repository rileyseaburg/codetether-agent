//! Recursive Language Model (RLM) processing
//!
//! Handles large contexts that exceed model context windows by:
//! 1. Loading context into a REPL environment as a variable
//! 2. Having the LLM write code to analyze it
//! 3. Supporting recursive sub-LM calls for semantic analysis
//!
//! Based on "Recursive Language Models" (Zhang et al. 2025)

pub mod chunker;
pub mod context_trace;
pub mod oracle;
pub mod repl;
pub mod router;
pub mod tools;

pub use chunker::{Chunk, ChunkOptions, ContentType, RlmChunker};
pub use context_trace::{ContextTrace, ContextEvent};
pub use oracle::{
    AstPayload, AstResult, FinalPayload, GeneratedQuery, GrepMatch, GrepOracle, GrepPayload,
    OracleResult, QueryTemplate, SemanticPayload, TemplateKind,
    TraceValidator, TreeSitterOracle, ValidatedTrace, VerificationMethod,
};
pub use repl::{ReplRuntime, RlmAnalysisResult, RlmExecutor, RlmRepl, SubQuery};
pub use router::{RlmRouter, RoutingContext, RoutingResult};
pub use tools::{RlmToolResult, dispatch_tool_call, rlm_tool_definitions};

use serde::{Deserialize, Serialize};

/// RLM processing statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RlmStats {
    pub input_tokens: usize,
    pub output_tokens: usize,
    pub iterations: usize,
    pub subcalls: usize,
    pub elapsed_ms: u64,
    pub compression_ratio: f64,
}

/// RLM processing result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RlmResult {
    pub processed: String,
    pub stats: RlmStats,
    pub success: bool,
    pub error: Option<String>,
}

/// RLM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RlmConfig {
    /// Mode: "auto", "off", or "always"
    #[serde(default = "default_mode")]
    pub mode: String,

    /// Threshold ratio of context window to trigger RLM (0.0-1.0)
    #[serde(default = "default_threshold")]
    pub threshold: f64,

    /// Maximum iterations for RLM processing
    #[serde(default = "default_max_iterations")]
    pub max_iterations: usize,

    /// Maximum recursive sub-calls
    #[serde(default = "default_max_subcalls")]
    pub max_subcalls: usize,

    /// Preferred runtime: "rust", "bun", or "python"
    #[serde(default = "default_runtime")]
    pub runtime: String,

    /// Model reference for root processing (provider:model)
    pub root_model: Option<String>,

    /// Model reference for subcalls (provider:model)
    pub subcall_model: Option<String>,
}

fn default_mode() -> String {
    "auto".to_string()
}

fn default_threshold() -> f64 {
    0.35
}

fn default_max_iterations() -> usize {
    15
}

fn default_max_subcalls() -> usize {
    50
}

fn default_runtime() -> String {
    "rust".to_string()
}

impl Default for RlmConfig {
    fn default() -> Self {
        Self {
            mode: default_mode(),
            threshold: default_threshold(),
            max_iterations: default_max_iterations(),
            max_subcalls: default_max_subcalls(),
            runtime: default_runtime(),
            root_model: None,
            subcall_model: None,
        }
    }
}
