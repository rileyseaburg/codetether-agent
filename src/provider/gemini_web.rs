//! Gemini Web provider  drives the Gemini chat UI's undocumented
//! BardChatUi endpoint using browser cookies stored in HashiCorp Vault.
//!
//! This provider reverse-engineers the same HTTP request the Gemini web app
//! sends so no official API key is required.  Authentication is entirely
//! cookie-based (Netscape cookies.txt from Cookie-Editor stored in Vault under
//! `secret/codetether/providers/gemini-web` as the `cookies` key).
//!
//! Supported models (Gemini 3 / 3.1 family):
//! - `gemini-web-fast`       Gemini 3 Fast (mode_id fbb127bbb056c959)
//! - `gemini-web-thinking`   Gemini 3 Thinking (mode_id 5bf011840784117a)
//! - `gemini-web-pro`        Gemini 3.1 Pro (mode_id 9d8ca3786ebdfbea)
//! - `gemini-web-deep-think` Gemini 3 Deep Think (mode_id e6fa609c3fa255c0)
//!
//! The model is selected via the `x-goog-ext-525001261-jspb` request header.

use super::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, Usage,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use futures::StreamExt as _;
use regex::Regex;
use reqwest::Client;
use serde_json::{Value, json};
use std::sync::OnceLock;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;

const GEMINI_ORIGIN: &str = "https://gemini.google.com";
const GEMINI_STREAM_PATH: &str =
    "/_/BardChatUi/data/assistant.lamda.BardFrontendService/StreamGenerate";

/// How long cached session tokens remain valid before re-scraping the home page.
const TOKEN_TTL: Duration = Duration::from_secs(20 * 60);

/// (model_id, mode_id, display_name, context_window)
const MODELS: &[(&str, &str, &str, usize)] = &[
    (
        "gemini-web-fast",
        "fbb127bbb056c959",
        "Gemini 3 Fast",
        1_048_576_usize,
    ),
    (
        "gemini-web-thinking",
        "5bf011840784117a",
        "Gemini 3 Thinking",
        1_048_576_usize,
    ),
    (
        "gemini-web-pro",
        "9d8ca3786ebdfbea",
        "Gemini 3.1 Pro",
        1_048_576_usize,
    ),
    (
        "gemini-web-deep-think",
        "e6fa609c3fa255c0",
        "Gemini 3 Deep Think",
        1_048_576_usize,
    ),
];

#[derive(Clone)]
struct SessionTokens {
    at_token: String,
    f_sid: String,
    bl: String,
    /// Path prefix extracted from the /u/N/ portion of the home-page redirect
    /// URL (e.g. "/u/1"). Empty for the primary account which uses no prefix.
    acct_prefix: String,
}

pub struct GeminiWebProvider {
    client: Client,
    /// Raw contents of cookies.txt (Netscape / Cookie-Editor export format)
    cookies: String,
    /// Cached session tokens with the instant they were fetched.
    token_cache: Mutex<Option<(SessionTokens, Instant)>>,
}

impl std::fmt::Debug for GeminiWebProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GeminiWebProvider")
            .field("cookies", &"[redacted]")
            .finish_non_exhaustive()
    }
}

impl GeminiWebProvider {
    /// Create a new provider from Netscape cookies.txt content.
    pub fn new(cookies: String) -> Result<Self> {
        let client = Client::builder()
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
                 AppleWebKit/537.36 (KHTML, like Gecko) \
                 Chrome/131.0.0.0 Safari/537.36",
            )
            .connect_timeout(std::time::Duration::from_secs(30))
            .timeout(std::time::Duration::from_secs(600))
            .build()
            .context("Failed to build reqwest client for GeminiWebProvider")?;
        Ok(Self {
            client,
            cookies,
            token_cache: Mutex::new(None),
        })
    }

    /// Convert a Netscape cookies.txt (7 tab-separated columns) into a
    /// `Cookie: name=value; ...` header string.
    ///
    /// Standard Netscape format columns:
    ///   0: domain  1: flag  2: path  3: secure  4: expiration  5: name  6: value
    ///
    /// If only 2 columns are present (simple `name\tvalue` format), those are
    /// used directly so the function remains backward-compatible.
    fn cookie_header(&self) -> String {
        self.cookies
            .lines()
            .filter_map(|line| {
                let t = line.trim();
                if t.is_empty() {
                    return None;
                }
                // "#HttpOnly_" prefix marks HttpOnly cookies in Netscape format —
                // strip it so the cookie is still included.  Pure comment lines
                // (e.g. the header block starting with "# Netscape") are skipped.
                let line = if let Some(rest) = t.strip_prefix("#HttpOnly_") {
                    rest
                } else if t.starts_with('#') {
                    return None;
                } else {
                    t
                };
                Some(line)
            })
            .filter_map(|line| {
                let parts: Vec<&str> = line.split('\t').collect();
                let (name, value) = if parts.len() >= 7 {
                    // Standard Netscape format
                    (parts[5].trim(), parts[6].trim())
                } else if parts.len() >= 2 {
                    // Simple name\tvalue format (backward compat)
                    (parts[0].trim(), parts[1].trim())
                } else {
                    return None;
                };
                if name.is_empty() {
                    return None;
                }
                Some(format!("{name}={value}"))
            })
            .collect::<Vec<_>>()
            .join("; ")
    }

    /// GET the Gemini home page and extract the three tokens we need:
    ///   - `at_token`  XSRF token (key `thykhd` or legacy `SNlM0e`)
    ///   - `f_sid`     session ID (`FdrFJe`)
    ///   - `bl`        build label (`cfb2h`)
    ///
    /// Regexes are compiled once and reused across calls via `OnceLock`.
    async fn get_session_tokens(&self) -> Result<SessionTokens> {
        static RE_NEW: OnceLock<Regex> = OnceLock::new();
        static RE_OLD: OnceLock<Regex> = OnceLock::new();
        static RE_BL: OnceLock<Regex> = OnceLock::new();
        static RE_SID: OnceLock<Regex> = OnceLock::new();

        let re_new = RE_NEW.get_or_init(|| Regex::new(r#""thykhd":"([^"]+)""#).unwrap());
        let re_old = RE_OLD.get_or_init(|| Regex::new(r#""SNlM0e":"([^"]+)""#).unwrap());
        let re_bl = RE_BL.get_or_init(|| Regex::new(r#""cfb2h":"([^"]+)""#).unwrap());
        let re_sid = RE_SID.get_or_init(|| Regex::new(r#""FdrFJe":"([^"]+)""#).unwrap());

        let cookie_hdr = self.cookie_header();
        let resp = self
            .client
            .get(GEMINI_ORIGIN)
            .header("Cookie", &cookie_hdr)
            .send()
            .await
            .context("Failed to fetch Gemini home page")?;

        // Extract the account prefix from the final URL after redirects.
        // Multi-account: redirects to /u/N/app  → prefix is "/u/N"
        // Primary account: redirects to /app    → prefix is ""
        let acct_prefix: String = {
            static RE_ACCT: OnceLock<Regex> = OnceLock::new();
            let re = RE_ACCT.get_or_init(|| Regex::new(r"(/u/\d+)/").unwrap());
            re.captures(resp.url().path())
                .map(|c| c[1].to_string())
                .unwrap_or_default()
        };
        tracing::debug!(acct_prefix = %acct_prefix, final_url = %resp.url(), "Gemini home page resolved");

        let html = resp
            .text()
            .await
            .context("Failed to read Gemini home page body")?;

        let at_token = if let Some(cap) = re_new.captures(&html) {
            cap[1].to_string()
        } else if let Some(cap) = re_old.captures(&html) {
            cap[1].to_string()
        } else {
            anyhow::bail!(
                "Could not find Gemini at-token (thykhd / SNlM0e)  \
                 cookies may be expired or invalid"
            );
        };

        let bl = re_bl
            .captures(&html)
            .map(|c| c[1].to_string())
            .unwrap_or_default();
        let f_sid = re_sid
            .captures(&html)
            .map(|c| c[1].to_string())
            .unwrap_or_default();

        if bl.is_empty() {
            tracing::warn!("Gemini bl token not found in home page — request may fail");
        }
        if f_sid.is_empty() {
            tracing::warn!("Gemini f_sid token not found in home page — request may fail");
        }

        Ok(SessionTokens {
            at_token,
            f_sid,
            bl,
            acct_prefix,
        })
    }

    /// Return session tokens, re-fetching only when the cached copy has
    /// exceeded `TOKEN_TTL` (20 minutes).
    async fn get_or_refresh_tokens(&self) -> Result<SessionTokens> {
        let mut cache = self.token_cache.lock().await;
        if let Some((ref tokens, fetched_at)) = *cache
            && fetched_at.elapsed() < TOKEN_TTL
        {
            return Ok(tokens.clone());
        }
        let fresh = self.get_session_tokens().await?;
        *cache = Some((fresh.clone(), Instant::now()));
        Ok(fresh)
    }

    /// Drop cached session tokens so the next request re-scrapes fresh values.
    async fn invalidate_tokens(&self) {
        let mut cache = self.token_cache.lock().await;
        *cache = None;
    }

    /// Build the `f.req` JSON payload — a two-element outer array whose
    /// second slot is a JSON-encoded 69-element inner array.
    ///
    /// Index layout matches the Python GemChat reference implementation.
    fn build_freq(prompt: &str) -> String {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let mut inner: Vec<Value> = vec![Value::Null; 69];
        // [0]  prompt tuple — single array, NOT double-wrapped
        inner[0] = json!([prompt, 0, null, null, null, null, 0]);
        // [1]  language
        inner[1] = json!(["en"]);
        // [2]  conversation continuation ids (empty strings for new convo)
        inner[2] = json!(["", "", "", null, null, null, null, null, null, ""]);
        // [3]-[5] null
        inner[6] = json!([1]);
        inner[7] = json!(1);
        // [8]-[9] null
        inner[10] = json!(1);
        inner[11] = json!(0);
        // [12] null
        // [13]-[16] null
        inner[17] = json!([[0]]);
        inner[18] = json!(0);
        // [19]-[26] null
        inner[27] = json!(1);
        // [28]-[29] null
        inner[30] = json!([4]);
        // [31]-[52] null
        inner[53] = json!(0);
        // [54]-[58] null
        inner[59] = json!("CD1035A5-0E0E-4B68-B744-23C2D8960DF5");
        // [60] null
        inner[61] = json!([]);
        // [62]-[65] null
        inner[66] = json!([ts, 0]);
        // [67] null
        inner[68] = json!(2);

        debug_assert_eq!(
            inner.len(),
            69,
            "f.req inner list must be exactly 69 elements"
        );

        let inner_json = serde_json::to_string(&inner).unwrap_or_default();
        serde_json::to_string(&json!([null, inner_json])).unwrap_or_default()
    }

    /// Walk streaming lines and return the longest parseable answer text.
    ///
    /// Wire format: each line is a JSON array of events:
    ///   `[["wrb.fr", key_or_null, inner_json_str, ...], ...]`
    /// The inner JSON has the response text at `inner[4][0][1][0]`.
    /// Gemini sends cumulative chunks; we take the longest (= most complete).
    fn extract_text(raw: &str) -> String {
        let mut best = String::new();
        for line in raw.lines() {
            let line = line.trim();
            if line.is_empty() || !line.starts_with('[') {
                continue;
            }
            let Ok(outer) = serde_json::from_str::<Value>(line) else {
                continue;
            };
            let Some(events) = outer.as_array() else {
                continue;
            };
            // Each element of the outer array is one event: ["wrb.fr", null, inner_str, ...]
            for event in events {
                let Some(ev) = event.as_array() else { continue };
                let Some(inner_str) = ev.get(2).and_then(Value::as_str) else {
                    continue;
                };
                if !inner_str.starts_with('[') {
                    continue;
                }
                let Ok(inner) = serde_json::from_str::<Value>(inner_str) else {
                    continue;
                };
                if let Some(text) = inner
                    .get(4)
                    .and_then(|v| v.get(0))
                    .and_then(|v| v.get(1))
                    .and_then(|v| v.get(0))
                    .and_then(Value::as_str)
                    && text.len() > best.len()
                {
                    best = text.to_string();
                }
            }
        }
        best
    }

    /// Extract Gemini protocol-level error code from SSE-like body lines.
    ///
    /// Looks for events like: `[["e",5,null,null,469]]`
    fn extract_protocol_error_code(raw: &str) -> Option<i64> {
        for line in raw.lines() {
            let line = line.trim();
            if line.is_empty() || !line.starts_with('[') {
                continue;
            }
            let Ok(events_val) = serde_json::from_str::<Value>(line) else {
                continue;
            };
            let Some(events) = events_val.as_array() else {
                continue;
            };
            for event in events {
                let Some(ev) = event.as_array() else { continue };
                let Some(kind) = ev.first().and_then(Value::as_str) else {
                    continue;
                };
                if kind != "e" {
                    continue;
                }
                if let Some(code) = ev.get(4).and_then(Value::as_i64) {
                    return Some(code);
                }
                if let Some(code) = ev.last().and_then(Value::as_i64) {
                    return Some(code);
                }
            }
        }
        None
    }

    /// Best-effort extraction of Gemini request id tokens like `r_52bc...`
    /// from protocol frames for diagnostics.
    fn extract_protocol_request_id(raw: &str) -> Option<String> {
        static RE_REQ_ID: OnceLock<Regex> = OnceLock::new();
        let re = RE_REQ_ID.get_or_init(|| Regex::new(r"(r_[A-Za-z0-9]+)").unwrap());
        re.captures(raw)
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
    }

    fn compact_body_snippet(raw: &str, max_chars: usize) -> String {
        raw.chars()
            .take(max_chars)
            .collect::<String>()
            .replace(['\n', '\r'], " ")
            .trim()
            .to_string()
    }

    fn format_protocol_error(code: i64, model: &str, raw: &str) -> String {
        let req_id = Self::extract_protocol_request_id(raw)
            .map(|id| format!(" request_id={id}."))
            .unwrap_or_default();
        let snippet = Self::compact_body_snippet(raw, 240);

        match code {
            469 => format!(
                "Gemini Web backend rejected the request (protocol code 469) for model `{model}`.{req_id} \
                 This is usually a transient web-backend/model-route issue or account entitlement mismatch. \
                 Try again, or switch to `gemini-web-thinking` / `gemini-web-fast`. Payload snippet: {snippet}"
            ),
            _ => format!(
                "Gemini Web backend returned protocol status code {code} for model `{model}`.{req_id} \
                 Payload snippet: {snippet}"
            ),
        }
    }

    /// Parse `<tool_call>{...}</tool_call>` JSON blocks from model text.
    ///
    /// Returns:
    /// - cleaned_text: original text with tool-call blocks removed
    /// - calls: parsed `(name, arguments_json_string)` tuples
    fn extract_tool_calls(text: &str) -> (String, Vec<(String, String)>) {
        fn normalize_tool_markup(input: &str) -> String {
            input
                // HTML-escaped tags from some markdown renderers
                .replace("&lt;", "<")
                .replace("&gt;", ">")
                // Backslash-escaped XML markers and markdown escapes
                .replace("\\<", "<")
                .replace("\\>", ">")
                .replace("\\_", "_")
        }

        static RE_TOOL_CALL_BLOCK: OnceLock<Regex> = OnceLock::new();
        static RE_TOOL_RESULT_BLOCK: OnceLock<Regex> = OnceLock::new();

        let re = RE_TOOL_CALL_BLOCK.get_or_init(|| {
            Regex::new(r"(?s)<tool_call>\s*(?:```(?:json)?\s*)?(\{.*?\})(?:\s*```)?\s*</tool_call>")
                .unwrap()
        });
        let re_tool_result = RE_TOOL_RESULT_BLOCK
            .get_or_init(|| Regex::new(r"(?s)<tool_result>.*?</tool_result>").unwrap());

        let normalized = normalize_tool_markup(text);

        let mut calls: Vec<(String, String)> = Vec::new();
        for captures in re.captures_iter(&normalized) {
            let Some(block_json) = captures.get(1).map(|m| m.as_str()) else {
                continue;
            };
            let Ok(value) = serde_json::from_str::<Value>(block_json) else {
                continue;
            };
            let Some(name) = value.get("name").and_then(Value::as_str) else {
                continue;
            };
            let name = name.trim();
            if name.is_empty() {
                continue;
            }
            let arguments = value.get("arguments").cloned().unwrap_or_else(|| json!({}));
            let args_json = serde_json::to_string(&arguments).unwrap_or_else(|_| "{}".to_string());
            calls.push((name.to_string(), args_json));
        }

        if calls.is_empty() {
            return (text.to_string(), Vec::new());
        }

        let without_calls = re.replace_all(&normalized, "").to_string();
        let cleaned = re_tool_result
            .replace_all(&without_calls, "")
            .trim()
            .to_string();
        (cleaned, calls)
    }

    /// Look up the `mode_id` string for a given model identifier.
    fn mode_id(model: &str) -> &'static str {
        MODELS
            .iter()
            .find(|(id, _, _, _)| *id == model)
            .map(|(_, mid, _, _)| *mid)
            .unwrap_or("fbb127bbb056c959")
    }

    /// Build a `RequestBuilder` for the StreamGenerate endpoint.
    async fn build_request(&self, prompt: &str, model: &str) -> Result<reqwest::RequestBuilder> {
        let tokens = self
            .get_or_refresh_tokens()
            .await
            .context("Failed to obtain Gemini session tokens")?;

        let cookie_hdr = self.cookie_header();
        let freq = Self::build_freq(prompt);
        let mode_id = Self::mode_id(model);

        let ext_header = {
            let v: Value = json!([
                1,
                null,
                null,
                null,
                mode_id,
                null,
                null,
                0,
                [4],
                null,
                null,
                3
            ]);
            serde_json::to_string(&v).unwrap_or_default()
        };

        let reqid = (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
            % 900_000
            + 100_000)
            .to_string();

        let endpoint = format!(
            "https://gemini.google.com{}{}",
            tokens.acct_prefix, GEMINI_STREAM_PATH
        );
        tracing::debug!(endpoint = %endpoint, "Gemini StreamGenerate endpoint");
        let url = reqwest::Url::parse_with_params(
            &endpoint,
            &[
                ("bl", tokens.bl.as_str()),
                ("f.sid", tokens.f_sid.as_str()),
                ("hl", "en"),
                ("pageId", "none"),
                ("_reqid", reqid.as_str()),
                ("rt", "c"),
            ],
        )
        .context("Failed to build Gemini endpoint URL")?;

        Ok(self
            .client
            .post(url)
            .header("Cookie", cookie_hdr)
            .header("X-Same-Domain", "1")
            .header("Origin", GEMINI_ORIGIN)
            .header("Referer", format!("{}/app", GEMINI_ORIGIN))
            .header("Accept", "*/*")
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("Cache-Control", "no-cache")
            .header("Pragma", "no-cache")
            .header("sec-fetch-dest", "empty")
            .header("sec-fetch-mode", "cors")
            .header("sec-fetch-site", "same-origin")
            .header("x-goog-ext-525001261-jspb", ext_header)
            .form(&[("f.req", freq), ("at", tokens.at_token)]))
    }

    /// Core blocking request: POST and collect the complete response.
    async fn ask(&self, prompt: &str, model: &str) -> Result<String> {
        for attempt in 0..=1 {
            let resp = self
                .build_request(prompt, model)
                .await?
                .send()
                .await
                .context("Failed to send request to Gemini StreamGenerate")?;

            if !resp.status().is_success() {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                if attempt == 0 {
                    tracing::warn!(
                        status = %status,
                        body_prefix = %body[..body.len().min(200)],
                        "Gemini request failed; invalidating cached tokens and retrying once"
                    );
                    self.invalidate_tokens().await;
                    continue;
                }
                anyhow::bail!(
                    "Gemini StreamGenerate returned HTTP {}: {}",
                    status,
                    &body[..body.len().min(500)]
                );
            }

            let body = resp
                .text()
                .await
                .context("Failed to read Gemini response body")?;
            let text = Self::extract_text(&body);
            if text.is_empty() {
                let protocol_code = Self::extract_protocol_error_code(&body);
                if attempt == 0 {
                    tracing::warn!(
                        body_prefix = %body[..body.len().min(200)],
                        "Gemini response had no parseable text; invalidating cached tokens and retrying once"
                    );
                    self.invalidate_tokens().await;
                    continue;
                }
                if let Some(code) = protocol_code {
                    anyhow::bail!(Self::format_protocol_error(code, model, &body));
                }
                anyhow::bail!(
                    "No text found in Gemini response for model `{}`. Payload snippet: {}",
                    model,
                    Self::compact_body_snippet(&body, 240)
                );
            }
            return Ok(text);
        }

        anyhow::bail!("Gemini request retry exhausted without a successful response")
    }
}

#[async_trait]
impl Provider for GeminiWebProvider {
    fn name(&self) -> &str {
        "gemini-web"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(MODELS
            .iter()
            .map(|(id, _, label, ctx)| ModelInfo {
                id: id.to_string(),
                name: label.to_string(),
                provider: "gemini-web".to_string(),
                context_window: *ctx,
                max_output_tokens: Some(65_536),
                supports_vision: false,
                supports_tools: false,
                supports_streaming: true,
                input_cost_per_million: Some(0.0),
                output_cost_per_million: Some(0.0),
            })
            .collect())
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let prompt = request
            .messages
            .iter()
            .map(|m| {
                let role = match m.role {
                    Role::System => "System",
                    Role::User => "User",
                    Role::Assistant => "Assistant",
                    Role::Tool => "Tool",
                };
                let text = m
                    .content
                    .iter()
                    .filter_map(|p| match p {
                        ContentPart::Text { text } => Some(text.clone()),
                        ContentPart::ToolCall {
                            name, arguments, ..
                        } => Some(format!("[Called tool: {name}({arguments})]")),
                        ContentPart::ToolResult { content, .. } => {
                            Some(format!("[Tool result]\n{content}"))
                        }
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("{role}: {text}")
            })
            .collect::<Vec<_>>()
            .join("\n");

        let text = self
            .ask(&prompt, &request.model)
            .await
            .context("Gemini Web completion failed")?;

        let (cleaned_text, parsed_tool_calls) = Self::extract_tool_calls(&text);
        let mut content: Vec<ContentPart> = Vec::new();
        if !cleaned_text.is_empty() {
            content.push(ContentPart::Text { text: cleaned_text });
        }

        for (idx, (name, arguments)) in parsed_tool_calls.iter().enumerate() {
            let ts = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis();
            content.push(ContentPart::ToolCall {
                id: format!("gwc_{ts}_{idx}"),
                name: name.clone(),
                arguments: arguments.clone(),
                thought_signature: None,
            });
        }

        if content.is_empty() {
            content.push(ContentPart::Text { text });
        }

        let finish_reason = if parsed_tool_calls.is_empty() {
            FinishReason::Stop
        } else {
            tracing::info!(
                model = %request.model,
                num_calls = parsed_tool_calls.len(),
                "Parsed tool calls from Gemini web text response"
            );
            FinishReason::ToolCalls
        };

        Ok(CompletionResponse {
            message: Message {
                role: Role::Assistant,
                content,
            },
            usage: Usage {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens: 0,
                cache_read_tokens: None,
                cache_write_tokens: None,
            },
            finish_reason,
        })
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        let prompt = request
            .messages
            .iter()
            .map(|m| {
                let role = match m.role {
                    Role::System => "System",
                    Role::User => "User",
                    Role::Assistant => "Assistant",
                    Role::Tool => "Tool",
                };
                let text = m
                    .content
                    .iter()
                    .filter_map(|p| match p {
                        ContentPart::Text { text } => Some(text.clone()),
                        ContentPart::ToolCall {
                            name, arguments, ..
                        } => Some(format!("[Called tool: {name}({arguments})]")),
                        ContentPart::ToolResult { content, .. } => {
                            Some(format!("[Tool result]\n{content}"))
                        }
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("{role}: {text}")
            })
            .collect::<Vec<_>>()
            .join("\n");

        let resp = self
            .build_request(&prompt, &request.model)
            .await?
            .send()
            .await
            .context("Failed to send streaming request to Gemini")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!(
                "Gemini StreamGenerate returned HTTP {}: {}",
                status,
                &body[..body.len().min(500)]
            );
        }

        // Process the byte stream and emit text deltas as chunks arrive.
        // Gemini sends cumulative text, so we track prev_len and emit only
        // the newly-arrived portion on each iteration.
        let byte_stream = resp.bytes_stream();
        let model_for_errors = request.model.clone();
        let (tx, rx) = futures::channel::mpsc::channel::<StreamChunk>(32);

        tokio::spawn(async move {
            futures::pin_mut!(byte_stream);
            let mut buf = String::new();
            let mut prev_len: usize = 0;
            let mut tx = tx;

            while let Some(chunk_result) = byte_stream.next().await {
                let Ok(bytes) = chunk_result else { break };
                let Ok(s) = std::str::from_utf8(&bytes) else {
                    continue;
                };
                buf.push_str(s);

                let current_text = Self::extract_text(&buf);
                if current_text.len() > prev_len {
                    let delta = current_text[prev_len..].to_string();
                    prev_len = current_text.len();
                    if tx.try_send(StreamChunk::Text(delta)).is_err() {
                        return; // receiver dropped
                    }
                }
            }

            // Final pass for any remaining delta in the last chunk
            let final_text = Self::extract_text(&buf);
            if final_text.len() > prev_len {
                let _ = tx.try_send(StreamChunk::Text(final_text[prev_len..].to_string()));
                prev_len = final_text.len();
            }

            if prev_len == 0 {
                if let Some(code) = Self::extract_protocol_error_code(&buf) {
                    let _ = tx.try_send(StreamChunk::Error(Self::format_protocol_error(
                        code,
                        &model_for_errors,
                        &buf,
                    )));
                    return;
                }
                if !buf.trim().is_empty() {
                    let _ = tx.try_send(StreamChunk::Error(format!(
                        "Gemini returned no text payload for model `{}`. Payload snippet: {}",
                        model_for_errors,
                        Self::compact_body_snippet(&buf, 240)
                    )));
                    return;
                }
            }
            let _ = tx.try_send(StreamChunk::Done { usage: None });
        });

        let stream = futures::stream::unfold(rx, |mut rx| async {
            use futures::StreamExt as _;
            rx.next().await.map(|chunk| (chunk, rx))
        });

        Ok(Box::pin(stream))
    }
}

#[cfg(test)]
mod tests {
    use super::GeminiWebProvider;

    #[test]
    fn extract_tool_calls_returns_calls_and_cleaned_text() {
        let text = r#"I will inspect the tree first.
<tool_call>
{"name":"tree","arguments":{"depth":3,"path":"."}}
</tool_call>
Then grep for key strings.
<tool_call>
{"name":"grep","arguments":{"pattern":"nextdoor","is_regex":false}}
</tool_call>"#;

        let (cleaned, calls) = GeminiWebProvider::extract_tool_calls(text);
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].0, "tree");
        assert!(calls[0].1.contains("\"depth\":3"));
        assert_eq!(calls[1].0, "grep");
        assert!(cleaned.contains("I will inspect the tree first."));
        assert!(cleaned.contains("Then grep for key strings."));
        assert!(!cleaned.contains("<tool_call>"));
    }

    #[test]
    fn extract_tool_calls_preserves_text_when_no_valid_blocks() {
        let text = "<tool_call>{not-json}</tool_call>";
        let (cleaned, calls) = GeminiWebProvider::extract_tool_calls(text);
        assert!(calls.is_empty());
        assert_eq!(cleaned, text);
    }

    #[test]
    fn extract_tool_calls_accepts_escaped_tool_markup() {
        let text = r#"I will invoke LSP now.
\<tool\_call\>
{"name":"lsp","arguments":{"action":"hover","file\_path":"api/src/a.ts","line":1,"column":1}}
\</tool\_call\>"#;

        let (cleaned, calls) = GeminiWebProvider::extract_tool_calls(text);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, "lsp");
        assert!(calls[0].1.contains("\"file_path\":\"api/src/a.ts\""));
        assert!(cleaned.contains("I will invoke LSP now."));
        assert!(!cleaned.contains("tool_call"));
    }

    #[test]
    fn extract_tool_calls_strips_tool_result_blocks_when_calls_present() {
        let text = r#"<tool_call>{"name":"bash","arguments":{"command":"pwd"}}</tool_call>
<tool_result>{"bash":"fake"}</tool_result>"#;
        let (cleaned, calls) = GeminiWebProvider::extract_tool_calls(text);
        assert_eq!(calls.len(), 1);
        assert!(cleaned.is_empty());
    }

    #[test]
    fn extract_protocol_error_code_reads_error_event() {
        let raw = r#"
)]}'
25
[["e",5,null,null,469]]
"#;
        assert_eq!(
            GeminiWebProvider::extract_protocol_error_code(raw),
            Some(469)
        );
    }

    #[test]
    fn extract_protocol_request_id_reads_wrapped_id() {
        let raw = r#"[["wrb.fr",null,"[null,[null,\"r_52bc718fbddfc769\"],null]"]]"#;
        assert_eq!(
            GeminiWebProvider::extract_protocol_request_id(raw).as_deref(),
            Some("r_52bc718fbddfc769")
        );
    }
}
