//! Native web search as an executable tool.
//!
//! [WebSearchTool] queries Bing HTML first and falls back to DuckDuckGo HTML
//! only when Bing fails (override with [`BACKEND_ENV`]). Output keeps the
//! historical `N. title` / `   URL: ...` / snippet lines and appends one
//! [`EVIDENCE_PREFIX`] JSON line recorded from the actual HTTP exchange:
//! query, backend, status, request ID, source URLs, and every attempt.
//!
//! Provider failures (challenge pages, rate limits, HTTP errors, transport
//! errors, unparseable pages) return an unsuccessful result carrying a
//! [`SearchFailure`] code. They are never reported as "No results found",
//! which is reserved for a provider page that legitimately lists nothing.
use anyhow::{Context, Result};
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde::Deserialize;
use serde_json::{Value, json};
use std::time::Duration;
mod attempt;
mod backend;
mod bing;
mod construct;
mod duckduckgo;
mod execute;
mod failure;
mod page;
mod params;
mod report;
mod result;
mod schema;
mod text;
mod tool;
pub use backend::{BACKEND_ENV, Backend, backend_order};
pub use construct::WebSearchTool;
pub use failure::SearchFailure;
use params::Params;
pub use report::EVIDENCE_PREFIX;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);

#[cfg(test)]
mod tests;
