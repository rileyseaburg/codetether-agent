//! Compare files or inspect Git differences through one executable tool.
//!
//! [DiffTool] preserves system diff output and the existing structured errors.
//! Schema, Git execution and direct file comparison have separate owners.
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::{Value, json};
mod construct;
mod execute;
mod files;
mod git;
mod schema;
mod tool;
pub use construct::DiffTool;
