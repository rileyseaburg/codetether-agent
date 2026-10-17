//! DuckDuckGo HTML web search as an executable tool.
//!
//! [WebSearchTool] retrieves search HTML, extracts titles/URLs/snippets and
//! renders the existing bounded result list without application dependencies.
use anyhow::{Context, Result};
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde::Deserialize;
use serde_json::{Value, json};
use std::time::Duration;
mod construct;
mod duckduckgo;
mod execute;
mod params;
mod result;
mod schema;
mod tool;
pub use construct::WebSearchTool;
use params::Params;
use result::SearchResult;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);
