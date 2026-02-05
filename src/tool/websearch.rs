//! Web Search Tool - Search the web using DuckDuckGo or configurable search API.

use super::{Tool, ToolResult};
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};
use std::time::Duration;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);

pub struct WebSearchTool {
    client: reqwest::Client,
}

impl Default for WebSearchTool {
    fn default() -> Self {
        Self::new()
    }
}

impl WebSearchTool {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .user_agent("CodeTether-Agent/1.0")
            .build()
            .expect("Failed to build HTTP client");
        Self { client }
    }

    async fn search_ddg(&self, query: &str, max_results: usize) -> Result<Vec<SearchResult>> {
        // DuckDuckGo HTML search (no API key required)
        let url = format!(
            "https://html.duckduckgo.com/html/?q={}",
            urlencoding::encode(query)
        );
        let resp = self.client.get(&url).send().await?;
        let html = resp.text().await?;

        let mut results = Vec::new();
        // Parse result links from DDG HTML
        let link_re =
            regex::Regex::new(r#"<a[^>]+class="result__a"[^>]+href="([^"]+)"[^>]*>([^<]+)</a>"#)?;
        let snippet_re = regex::Regex::new(r#"<a[^>]+class="result__snippet"[^>]*>([^<]+)</a>"#)?;

        let links: Vec<_> = link_re.captures_iter(&html).collect();
        let snippets: Vec<_> = snippet_re.captures_iter(&html).collect();

        for (i, cap) in links.iter().take(max_results).enumerate() {
            let url = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let title = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let snippet = snippets
                .get(i)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str())
                .unwrap_or("");

            // DDG wraps URLs, extract actual URL
            let actual_url = if url.contains("uddg=") {
                url.split("uddg=")
                    .nth(1)
                    .and_then(|s| urlencoding::decode(s.split('&').next().unwrap_or("")).ok())
                    .map(|s| s.into_owned())
                    .unwrap_or_else(|| url.to_string())
            } else {
                url.to_string()
            };

            results.push(SearchResult {
                title: html_escape::decode_html_entities(title).to_string(),
                url: actual_url,
                snippet: html_escape::decode_html_entities(snippet).to_string(),
            });
        }
        Ok(results)
    }
}

#[derive(Debug, Clone)]
struct SearchResult {
    title: String,
    url: String,
    snippet: String,
}

#[derive(Deserialize)]
struct Params {
    query: String,
    #[serde(default = "default_max")]
    max_results: usize,
}

fn default_max() -> usize {
    5
}

#[async_trait]
impl Tool for WebSearchTool {
    fn id(&self) -> &str {
        "websearch"
    }
    fn name(&self) -> &str {
        "Web Search"
    }
    fn description(&self) -> &str {
        "Search the web for information. Returns titles, URLs, and snippets."
    }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "query": {"type": "string", "description": "Search query"},
                "max_results": {"type": "integer", "default": 5, "description": "Max results to return"}
            },
            "required": ["query"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;

        if p.query.trim().is_empty() {
            return Ok(ToolResult::error("Query cannot be empty"));
        }

        let results = self.search_ddg(&p.query, p.max_results).await?;

        if results.is_empty() {
            return Ok(ToolResult::success("No results found".to_string()));
        }

        let output = results
            .iter()
            .enumerate()
            .map(|(i, r)| {
                format!(
                    "{}. {}\n   URL: {}\n   {}",
                    i + 1,
                    r.title,
                    r.url,
                    r.snippet
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n");

        Ok(ToolResult::success(output).with_metadata("count", json!(results.len())))
    }
}
