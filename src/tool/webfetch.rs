//! Web Fetch Tool - Fetches content from URLs and converts HTML to markdown/text/html.

use super::{Tool, ToolResult};
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};
use std::time::Duration;

#[allow(dead_code)]
const MAX_CONTENT_LENGTH: usize = 10 * 1024 * 1024;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

pub struct WebFetchTool {
    client: reqwest::Client,
}

impl Default for WebFetchTool {
    fn default() -> Self {
        Self::new()
    }
}

impl WebFetchTool {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .user_agent("CodeTether-Agent/1.0")
            .redirect(reqwest::redirect::Policy::limited(5))
            .build()
            .expect("Failed to build HTTP client");
        Self { client }
    }

    fn html_to_markdown(&self, html: &str) -> String {
        let mut result = html.to_string();
        let patterns = [
            (r"<h1[^>]*>(.*?)</h1>", "# $1\n"),
            (r"<h2[^>]*>(.*?)</h2>", "## $1\n"),
            (r"<h3[^>]*>(.*?)</h3>", "### $1\n"),
            (r"<p[^>]*>(.*?)</p>", "$1\n\n"),
            (r"<(strong|b)[^>]*>(.*?)</\1>", "**$2**"),
            (r"<(em|i)[^>]*>(.*?)</\1>", "*$2*"),
            (r"<code[^>]*>(.*?)</code>", "`$1`"),
            (r"<li[^>]*>(.*?)</li>", "- $1\n"),
        ];
        for (pat, rep) in patterns {
            if let Ok(re) = regex::Regex::new(pat) {
                result = re.replace_all(&result, rep).to_string();
            }
        }
        result = regex::Regex::new(r"<[^>]+>")
            .unwrap()
            .replace_all(&result, "")
            .to_string();
        result
            .replace("&nbsp;", " ")
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
    }

    fn html_to_text(&self, html: &str) -> String {
        let md = self.html_to_markdown(html);
        md.replace("**", "")
            .replace("*", "")
            .replace("`", "")
            .replace("# ", "")
    }

    fn is_html(&self, ct: &str, body: &str) -> bool {
        ct.contains("text/html") || body.trim().starts_with('<')
    }
}

#[derive(Deserialize)]
struct Params {
    url: String,
    #[serde(default = "default_fmt")]
    format: String,
}
fn default_fmt() -> String {
    "markdown".into()
}

#[async_trait]
impl Tool for WebFetchTool {
    fn id(&self) -> &str {
        "webfetch"
    }
    fn name(&self) -> &str {
        "Web Fetch"
    }
    fn description(&self) -> &str {
        "Fetch content from URL, convert HTML to markdown/text/html."
    }
    fn parameters(&self) -> Value {
        json!({"type":"object","properties":{"url":{"type":"string"},"format":{"type":"string","enum":["markdown","text","html"],"default":"markdown"}},"required":["url"]})
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;
        let url = p.url.parse::<reqwest::Url>().context("Invalid URL")?;
        if url.scheme() != "http" && url.scheme() != "https" {
            return Ok(ToolResult::error("Only HTTP/HTTPS supported"));
        }
        let resp = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        if !resp.status().is_success() {
            return Ok(ToolResult::error(format!("HTTP {}", resp.status())));
        }
        let ct = resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_lowercase();
        let body = resp.text().await.context("Failed to read body")?;
        let content = match p.format.as_str() {
            "html" => body,
            "text" => {
                if self.is_html(&ct, &body) {
                    self.html_to_text(&body)
                } else {
                    body
                }
            }
            _ => {
                if self.is_html(&ct, &body) {
                    self.html_to_markdown(&body)
                } else {
                    body
                }
            }
        };
        Ok(ToolResult::success(content).with_metadata("url", json!(p.url)))
    }
}
