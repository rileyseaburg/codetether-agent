//! Literal and regex file search with bounded resource use.
//!
//! [GrepTool] preserves the application's grep semantics, including literal
//! matching by default, ignore-aware traversal and partial-result metadata.
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use ignore::WalkBuilder;
use regex::Regex;
use serde_json::{Value, json};
use std::time::{Duration, Instant};
mod construct;
mod environment;
mod execute;
mod lines;
mod options;
mod read_file;
mod render;
mod scan;
mod scanned;
mod schema;
mod select;
mod tool;
pub use construct::GrepTool;
use environment::{env_u64, env_usize, remaining_duration};
mod options_data;
use options_data::Options;
mod scan_file;
use read_file::ReadOutcome;
use scanned::Scanned;

const DEFAULT_GREP_LIMIT: usize = 50;

const MAX_GREP_LIMIT: usize = 500;

const DEFAULT_GREP_TIMEOUT_SECS: u64 = 15;

const MAX_GREP_TIMEOUT_SECS: u64 = 120;

const DEFAULT_GREP_MAX_SCANNED_FILES: usize = 10_000;

const DEFAULT_GREP_MAX_FILE_BYTES: u64 = 1024 * 1024;
#[cfg(test)]
mod tests;
