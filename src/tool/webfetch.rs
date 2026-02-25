//! Web Fetch Tool - Fetches content from URLs and converts HTML to markdown/text/html.

use super::{Tool, ToolResult};
use anyhow::{Context, Result};
use async_trait::async_trait;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use serde_json::{Value, json};
use std::time::Duration;

#[allow(dead_code)]
const MAX_CONTENT_LENGTH: usize = 10 * 1024 * 1024;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

const DEFAULT_MAX_CHARS: usize = 200_000;

static RE_STRIP_SCRIPT_STYLE: Lazy<Regex> = Lazy::new(|| {
    // DEPRECATED: kept for backwards compatibility with old builds.
    // NOTE: The Rust `regex` crate does NOT support backreferences, so we do
    // not rely on this regex for correctness. See `preprocess_html()`.
    Regex::new(r"(?is)<script[^>]*>.*?</script>").expect("invalid regex")
});

static RE_STRIP_HTML_COMMENTS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?is)<!--.*?-->").expect("invalid regex"));

static RE_STRIP_SCRIPT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?is)<script[^>]*>.*?</script>").expect("invalid regex"));
static RE_STRIP_STYLE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?is)<style[^>]*>.*?</style>").expect("invalid regex"));
static RE_STRIP_NOSCRIPT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?is)<noscript[^>]*>.*?</noscript>").expect("invalid regex"));
static RE_STRIP_SVG: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?is)<svg[^>]*>.*?</svg>").expect("invalid regex"));
static RE_STRIP_CANVAS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?is)<canvas[^>]*>.*?</canvas>").expect("invalid regex"));
static RE_STRIP_IFRAME: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?is)<iframe[^>]*>.*?</iframe>").expect("invalid regex"));

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

    fn preprocess_html(&self, html: &str) -> String {
        // Remove the highest-noise/highest-token sections first.
        // Many docs sites embed large JS bundles and assistant widgets inside
        // <script> tags; we do not want that in agent context.
        let mut s = html.to_string();

        // Strip common "noise" blocks that otherwise dominate the output.
        // (Rust regex has no backreferences, so we do them explicitly.)
        for re in [
            &*RE_STRIP_SCRIPT,
            &*RE_STRIP_STYLE,
            &*RE_STRIP_NOSCRIPT,
            &*RE_STRIP_SVG,
            &*RE_STRIP_CANVAS,
            &*RE_STRIP_IFRAME,
        ] {
            s = re.replace_all(&s, "").to_string();
        }

        s = RE_STRIP_HTML_COMMENTS.replace_all(&s, "").to_string();
        s
    }

    fn html_to_markdown(&self, html: &str) -> String {
        let html = self.preprocess_html(html);
        let mut result = html;
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
    #[serde(default = "default_max_chars")]
    max_chars: usize,
}
fn default_fmt() -> String {
    "markdown".into()
}

fn default_max_chars() -> usize {
    DEFAULT_MAX_CHARS
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
        json!({
            "type": "object",
            "properties": {
                "url": {"type": "string"},
                "format": {"type": "string", "enum": ["markdown", "text", "html"], "default": "markdown"},
                "max_chars": {
                    "type": "integer",
                    "minimum": 1000,
                    "default": DEFAULT_MAX_CHARS,
                    "description": "Maximum number of characters to return (safety limit to avoid overflowing the model context window)."
                }
            },
            "required": ["url"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;
        let url = p.url.parse::<reqwest::Url>().context("Invalid URL")?;
        if url.scheme() != "http" && url.scheme() != "https" {
            return Ok(ToolResult::error("Only HTTP/HTTPS supported"));
        }

        crate::tls::ensure_rustls_crypto_provider();

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
        let bytes = resp.bytes().await.context("Failed to read body")?;
        if bytes.len() > MAX_CONTENT_LENGTH {
            return Ok(ToolResult::error(format!(
                "Content too large ({} bytes > {} max)",
                bytes.len(),
                MAX_CONTENT_LENGTH
            )));
        }
        let body = String::from_utf8_lossy(&bytes).to_string();
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

        let mut out = content;
        let truncated = if out.chars().count() > p.max_chars {
            // Keep head + tail so navigation/footer and disclaimers can still be spotted.
            let head_chars = (p.max_chars as f64 * 0.70) as usize;
            let tail_chars = (p.max_chars as f64 * 0.20) as usize;

            let head: String = out.chars().take(head_chars).collect();
            let tail: String = out
                .chars()
                .rev()
                .take(tail_chars)
                .collect::<String>()
                .chars()
                .rev()
                .collect();

            let total_chars = out.chars().count();
            out = format!(
                "{}\n\n[... truncated {} chars (max_chars={}) ...]\n\n{}",
                head,
                total_chars.saturating_sub(head_chars + tail_chars),
                p.max_chars,
                tail
            );
            true
        } else {
            false
        };

        Ok(ToolResult::success(out)
            .with_metadata("url", json!(p.url))
            .with_metadata("format", json!(p.format))
            .with_metadata("truncated", json!(truncated))
            .with_metadata("max_chars", json!(p.max_chars)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_to_markdown_strips_script_content() {
        let tool = WebFetchTool::new();
        let html = r#"<html><head><title>x</title></head><body>
<h1>Hello</h1>
<script>window.__assistant_state = { open: true }; function big(){ return 1; }</script>
<p>World</p>
</body></html>"#;

        let md = tool.html_to_markdown(html);
        assert!(md.contains("Hello"));
        assert!(md.contains("World"));
        assert!(!md.contains("__assistant_state"));
        assert!(!md.contains("function big"));
    }
}
