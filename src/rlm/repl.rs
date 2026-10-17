//! Host construction and compatibility API for legacy RLM execution.
//!
//! [RlmExecutor] retains the historical API and supplies the application's
//! optional tool router to the shared executor. REPL context and process
//! types remain available through their existing re-exports.
use crate::cognition::tool_router::{ToolCallRouter, ToolRouterConfig};
use crate::provider::Provider;
use crate::rlm::{RlmAnalysisResult, context_trace::ContextTraceSummary, oracle::TraceStep};
use anyhow::Result;
pub use codetether_rlm::external_repl::ExternalRepl;
use codetether_rlm::legacy_executor::ResponseRewriter;
pub use codetether_rlm::repl::{DslResult, ReplResult, ReplRuntime, RlmRepl};
use std::sync::Arc;
mod analysis_loop;
mod executor_access;
mod executor_construct;
mod executor_state;
mod rewriter;
pub use executor_state::RlmExecutor;
