//! Core read, write, list and glob tools with shared validation and telemetry.
//!
//! [ReadTool] selects file lines, [WriteTool] validates and records mutations,
//! [ListTool] lists entries, and [GlobTool] searches without following links.
use anyhow::Result;
use async_trait::async_trait;
use codetether_telemetry::{FileChange, TOOL_EXECUTIONS, ToolExecution, record_persistent};
use codetether_tool_core::{Tool, ToolResult};
use serde_json::{Value, json};
use std::{path::PathBuf, time::Instant};
use tokio::fs;
mod file_write_args;
mod glob_construct;
mod glob_pattern;
mod glob_schema;
mod glob_search;
mod glob_tool;
mod glob_walk;
mod list;
mod list_construct;
mod list_schema;
mod list_tool;
mod read;
mod read_construct;
mod read_schema;
mod read_telemetry;
mod read_tool;
mod write;
mod write_construct;
mod write_schema;
mod write_telemetry;
mod write_tool;
pub use glob_construct::GlobTool;
pub use list_construct::ListTool;
pub use read_construct::ReadTool;
pub use write_construct::WriteTool;
