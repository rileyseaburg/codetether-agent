//! Regex-based workspace code search with contextual result rendering.
//!
//! [CodeSearchTool] prunes excluded directories, recognizes source extensions
//! and renders matching lines with bounded surrounding context.
use anyhow::{Context, Result};
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use codetether_util::workspace_scan::path_has_pruned_component;
use serde::Deserialize;
use serde_json::{Value, json};
use std::path::PathBuf;
use walkdir::WalkDir;
mod collect;
mod construct;
mod execute;
mod filter;
mod params;
mod record;
mod render;
mod schema;
mod search_file;
mod tool;
pub use construct::CodeSearchTool;
use params::Params;
use record::Match;

const MAX_RESULTS: usize = 50;

const MAX_CONTEXT_LINES: usize = 3;
