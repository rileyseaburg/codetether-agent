//! HTTP content fetching, conversion and bounded tool output.
//!
//! [WebFetchTool] fetches HTTP/HTTPS URLs, applies repository-scoped GitHub
//! credentials and redaction, converts HTML and bounds returned content.
use anyhow::{Context, Result};
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::{Value, json};
use std::time::Duration;
mod construct;
mod convert;
mod detect;
mod execute;
mod github;
mod markdown;
mod params;
mod patterns;
mod preprocess;
mod result;
mod schema;
mod text;
mod tool;
mod truncate;
pub use construct::WebFetchTool;
use patterns::*;

#[allow(dead_code)]
const MAX_CONTENT_LENGTH: usize = 10 * 1024 * 1024;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

const DEFAULT_MAX_CHARS: usize = 200_000;
#[cfg(test)]
mod tests;
