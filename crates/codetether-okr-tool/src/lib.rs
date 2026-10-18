//! OKR tool schemas, validation and repository action dispatch.
//!
//! [OkrTool] uses one lazily initialized repository. Objective, run and query
//! actions have focused modules and preserve the existing response contracts.
use anyhow::{Context, Result};
use async_trait::async_trait;
use codetether_okr::{KeyResult, Okr, OkrRepository, OkrRun, OkrRunStatus, OkrStatus};
use codetether_tool_core::{Tool, ToolResult};
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::Arc;
use tokio::sync::OnceCell;
use uuid::Uuid;
mod construct;
mod create_okr;
mod create_run;
mod delete_okr;
mod delete_run;
mod execute;
mod get_okr;
mod get_run;
mod list_okrs;
mod list_runs;
mod params;
mod parse;
mod query_okrs;
mod query_runs;
mod repository;
mod schema;
mod stats;
mod tool;
mod update_okr;
mod update_run;
pub use construct::OkrTool;
use params::Params;
use parse::{parse_okr_status, parse_run_status, parse_uuid};
use repository::get_repo;

/// Lazily-initialized shared OKR repository
static OKR_REPO: OnceCell<Arc<OkrRepository>> = OnceCell::const_new();
