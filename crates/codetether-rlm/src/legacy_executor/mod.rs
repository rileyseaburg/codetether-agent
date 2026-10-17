//! Legacy RLM analysis with shared providers and optional host response rewriting.
//!
//! Provides a REPL-like environment where context is loaded as a variable
//! and the LLM can execute code to analyze it.
//!
//! Key feature: llm_query() function for recursive sub-LM calls.
//!
//! The executor sends RLM tool definitions alongside the analysis prompt.
//! A caller-supplied [`ResponseRewriter`] can rewrite responses before dispatch.
//! Responses containing structured
//! `ContentPart::ToolCall` entries are dispatched directly instead of being
//! regex-parsed from code blocks (the legacy DSL path is kept as a fallback).

mod analysis_dsl_step;
mod analysis_loop;
mod analysis_prepare;
mod analysis_run;
mod analysis_step;
mod analysis_structured_step;
use analysis_run::AnalysisRun;
mod analysis_finish;
mod analysis_initial_trace;
mod analysis_messages;
mod analysis_request;
mod analysis_response_trace;
mod analysis_rewrite;
mod analysis_summary;
mod dsl_execute;
mod dsl_final;
mod dsl_log;
mod dsl_output;
mod dsl_reject;
mod executor_access;
mod executor_construct;
mod executor_state;
mod executor_support;
mod structured_batch;
mod structured_call;
use dsl_final::DslControl;
mod query_dsl;
mod query_sequence;
mod query_structured;
mod structured_final;
mod structured_output;
mod structured_query;
mod structured_reject;
mod structured_unknown;

pub use crate::external_repl::ExternalRepl;
use crate::legacy_support::truncate_with_ellipsis;
pub use crate::repl::{DslResult, ReplResult, ReplRuntime, RlmRepl};
use anyhow::Result;
use std::sync::Arc;

use codetether_provider_core::{CompletionRequest, ContentPart, Message, Provider, Role};

mod rewriter;
pub use rewriter::ResponseRewriter;

use crate::context_trace::{ContextEvent, ContextTrace, ContextTraceSummary};
use crate::oracle::{GrepOracle, QueryType, TraceStep};
use crate::tools::{RlmToolResult, dispatch_tool_call, rlm_tool_definitions};
use crate::{RlmAnalysisResult, SubQuery};
pub use executor_state::RlmExecutor;
