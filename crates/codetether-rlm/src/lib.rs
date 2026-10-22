//! # Recursive Language Model (RLM)
//!
//! Core types, oracle, chunker, event definitions, and the routing
//! subsystem for Recursive Language Model processing.
//!
//! [provider_adapter] bridges shared provider contracts into the RLM interface.
//! [repl] owns in-memory context operations and DSL evaluation, while
//! [external_repl] owns Bun/Node process I/O. [legacy_executor] runs analysis
//! with optional caller-supplied response rewriting. Session event adapters
//! remain in the application. The router is
//! provided here behind the [`RouterHost`] trait so the host can
//! inject its concrete tool-dispatch implementation.

pub mod capability;
pub mod chunker;
pub mod config;
pub mod context_index;
pub mod context_trace;
pub mod engine;
pub mod events;
pub mod model_select;
pub mod oracle;
pub mod result;
pub mod router;
pub mod stats;
pub mod text_budget;
pub mod traits;
pub mod types;

pub use chunker::{Chunk, ChunkOptions, ContentType, RlmChunker};
pub use config::RlmConfig;
pub use context_index::{ContextIndex, EvidenceKind, EvidenceRecord, PlanIntent, RetrievalPlan};
pub use events::{RlmCompletion, RlmOutcome, RlmProgressEvent, RlmSubcallFallback, S3Config};
pub use model_select::{
    RLM_MODEL_ENV, RlmModelChoice, RlmModelPurpose, RlmModelSource, select_rlm_model,
    select_rlm_model_with_env,
};
pub use oracle::{
    AstCapture, AstPayload, AstResult, FinalPayload, GeneratedQuery, GrepMatch, GrepOracle,
    GrepPayload, GrepVerification, OracleResult, OracleTracePersistResult, OracleTraceRecord,
    OracleTraceStorage, OracleTraceSyncStats, QueryTemplate, SemanticPayload, TemplateKind,
    TraceStep, TraceValidator, TreeSitterOracle, TreeSitterVerification, ValidatedTrace,
    VerificationMethod,
};
pub use result::RlmResult;
pub use router::{
    CrateAutoProcessContext, HostToolResult, IntoCrateCtx, ProcessProgress, RouterHost,
    RoutingContext, RoutingResult, auto_process, extract_final, fallback_result, should_route,
    smart_truncate,
};
pub use stats::RlmStats;
pub use types::{RlmAnalysisResult, SubQuery};

/// Shared-provider adapter and request/response conversion.
pub mod provider_adapter;

/// In-memory REPL context operations and DSL evaluation.
pub mod repl;

/// RLM tool schemas, bounded outputs, and in-memory dispatch.
pub mod tools;

/// External Bun/Node process wrapper for REPL commands.
pub mod external_repl;

/// Legacy executor text decoding and trace-evidence helpers.
pub mod legacy_support;

/// Legacy analysis executor with caller-supplied response rewriting.
pub mod legacy_executor;

/// Router-backed executor preserving the root application compatibility API.
pub mod router_executor;

/// Host-loaded provider resolution for configured RLM model choices.
pub mod model_resolution;
