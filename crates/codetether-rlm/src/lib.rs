//! # Recursive Language Model (RLM)
//!
//! Core types, oracle, chunker, event definitions, and the routing
//! subsystem for Recursive Language Model processing.
//!
//! **Driver modules** (`repl`, `tools`) remain in `codetether-agent`
//! due to tight coupling with `Provider`/`SessionBus`. The router is
//! provided here behind the [`RouterHost`] trait so the host can
//! inject its concrete tool-dispatch implementation.

pub mod capability;
pub mod chunker;
pub mod config;
pub mod context_trace;
pub mod events;
pub mod oracle;
pub mod result;
pub mod router;
pub mod stats;
pub mod traits;
pub mod types;

pub use chunker::{Chunk, ChunkOptions, ContentType, RlmChunker};
pub use config::RlmConfig;
pub use events::{RlmCompletion, RlmOutcome, RlmProgressEvent, RlmSubcallFallback, S3Config};
pub use oracle::{
    AstPayload, AstResult, FinalPayload, GeneratedQuery, GrepMatch, GrepOracle, GrepPayload,
    GrepVerification, OracleResult, OracleTracePersistResult, OracleTraceRecord,
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
