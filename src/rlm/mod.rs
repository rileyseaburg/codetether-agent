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
pub use context_trace::{ContextEvent, ContextTrace};
pub use oracle::{
    AstPayload, AstResult, FinalPayload, GeneratedQuery, GrepMatch, GrepOracle, GrepPayload,
    GrepVerification, OracleResult, OracleTracePersistResult, OracleTraceRecord,
    OracleTraceStorage, OracleTraceSyncStats, QueryTemplate, SemanticPayload, TemplateKind,
    TraceStep, TraceValidator, TreeSitterOracle, TreeSitterVerification, ValidatedTrace,
    VerificationMethod,
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

/// RLM processing result.
///
/// The `trace` field is populated when the caller supplied a
/// [`crate::session::SessionBus`] (or otherwise opted in) so downstream
/// consumers — the TUI `/rlm` view, the JSONL flywheel, trace-driven
/// tuning jobs — can reconstruct the iteration-by-iteration behaviour
/// of the loop after the fact.
///
/// `trace_id` is always generated for a run (even when no bus is
/// attached) and is echoed in the matching
/// [`crate::session::SessionEvent::RlmComplete`] event. Callers who
/// supplied a bus can use it to correlate the durable completion
/// record with this returned value.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::rlm::{RlmResult, RlmStats};
///
/// let r = RlmResult {
///     processed: "summary".into(),
///     stats: RlmStats::default(),
///     success: true,
///     error: None,
///     trace: None,
///     trace_id: None,
/// };
/// assert!(r.success);
/// assert!(r.trace.is_none());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RlmResult {
    /// The final text produced by the loop (summary or answer).
    pub processed: String,
    /// Aggregate statistics for the run.
    pub stats: RlmStats,
    /// `true` when the loop converged within its iteration budget.
    pub success: bool,
    /// Populated when `success` is `false` — a short diagnostic.
    pub error: Option<String>,
    /// Optional per-iteration event trace. Serialised only when present
    /// so existing on-disk `RlmResult` records stay compatible.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace: Option<context_trace::ContextTrace>,
    /// Identifier echoed on the matching `RlmComplete` bus event.
    /// `None` for on-disk records written before this field existed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<uuid::Uuid>,
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

    /// Maximum iterations for RLM processing.
    ///
    /// # Semantics
    ///
    /// An "iteration" is one full router step: system prompt + tools →
    /// LLM round-trip → tool calls → results → next LLM round-trip.
    /// The loop terminates in one of four ways, mapped directly to
    /// [`RlmOutcome`](crate::session::RlmOutcome) on the emitted
    /// [`RlmComplete`](crate::session::SessionEvent::RlmComplete) event:
    ///
    /// | Termination condition                                 | Outcome       |
    /// |-------------------------------------------------------|---------------|
    /// | Model emitted a `FINAL:` marker                        | `Converged`   |
    /// | `max_iterations` reached without `FINAL:`              | `Exhausted`   |
    /// | Provider or tool raised an error                       | `Failed`      |
    /// | The caller's `AbortHandle` fired                       | `Aborted`     |
    ///
    /// `Exhausted` is **not** an error — the partial result is still
    /// returned and the caller decides whether to retry with a higher
    /// limit, fall back to chunk compression, or surface the partial
    /// answer to the user. Session-level context compaction (see
    /// [`crate::session::helper::compression`]) treats `Exhausted` the
    /// same as success and re-uses the summary it produced.
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
