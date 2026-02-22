//! Gemini Web provider — drives the Gemini chat UI's undocumented
//! BardChatUi endpoint using browser cookies stored in HashiCorp Vault.
//!
//! This provider reverse-engineers the same HTTP request the Gemini web app
//! sends so no official API key is required.  Authentication is entirely
//! cookie-based (tab-separated Cookie-Editor export stored in Vault under
//! `secret/codetether/providers/gemini-web` as the `cookies` key).
//!
//! Supported models (Gemini 3 / 3.1 family):
//! - `gemini-web-fast`     → Gemini 3 Fast (mode_id fbb127bbb056c959)
//! - `gemini-web-thinking` → Gemini 3 Thinking (mode_id 5bf011840784117a)
//! - `gemini-web-pro`      → Gemini 3.1 Pro (mode_id 9d8ca3786ebdfbea)
//!
//! The model is selected via the `x-goog-ext-525001261-jspb` request header.

use super::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, Usage,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use regex::Regex;
use reqwest::Client;
use serde_json::{Value, json};
use std::time::{SystemTime, UNIX_EPOCH};

const GEMINI_ORIGIN: &str = "https://gemini.google.com";
const GEMINI_ENDPOINT: &str = "https://gemini.google.com/u/1/_/BardChatUi/data/assistant.lamda.BardFrontendService/StreamGenerate";

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
];

struct SessionTokens {
    at_token: String,
    f_sid: String,
    bl: String,
}

pub struct GeminiWebProvider {
    client: Client,
    /// Raw contents of cookies.txt (tab-separated, Cookie-Editor export format)
    cookies: String,
}

impl std::fmt::Debug for GeminiWebProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GeminiWebProvider")
            .field("cookies", &"[redacted]")
            .finish()
    }
}

impl GeminiWebProvider {
    /// Create a new provider from tab-separated cookies.txt content.
    pub fn new(cookies: String) -> Result<Self> {
        let client = Client::builder()
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
                 AppleWebKit/537.36 (KHTML, like Gecko) \
                 Chrome/131.0.0.0 Safari/537.36",
            )
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .context("Failed to build reqwest client for GeminiWebProvider")?;
        Ok(Self { client, cookies })
    }

    /// Convert tab-separated cookies.txt into a `Cookie: name=value; ...` string.
    ///
    /// Supports the format exported by the Cookie-Editor browser extension:
    ///   name<TAB>value<TAB>domain<TAB>path<TAB>...
    fn cookie_header(&self) -> String {
        self.cookies
            .lines()
            .filter(|l| {
                let t = l.trim();
                !t.is_empty() && !t.starts_with('#')
            })
            .filter_map(|line| {
                let mut parts = line.splitn(3, '\t');
                let name = parts.next()?.trim();
                let value = parts.next()?.trim();
                if name.is_empty() {
                    return None;
                }
                Some(format!("{name}={value}"))
            })
            .collect::<Vec<_>>()
            .join("; ")
    }

    /// GET the Gemini home page and extract the three tokens we need:
    ///   - `at_token` — XSRF token (key `thykhd` or legacy `SNlM0e`)
    ///   - `f_sid`    — session ID (`FdrFJe`)
    ///   - `bl`       — build label (`cfb2h`)
    async fn get_session_tokens(&self) -> Result<SessionTokens> {
        let cookie_hdr = self.cookie_header();
        let html = self
            .client
            .get(GEMINI_ORIGIN)
            .header("Cookie", &cookie_hdr)
            .send()
            .await
            .context("Failed to fetch Gemini home page")?
            .text()
            .await
            .context("Failed to read Gemini home page body")?;

        // at-token: current key is `thykhd`, fall back to the old `SNlM0e`
        let at_token = {
            let re_new = Regex::new(r#""thykhd":"([^"]+)""#).unwrap();
            let re_old = Regex::new(r#""SNlM0e":"([^"]+)""#).unwrap();
            if let Some(cap) = re_new.captures(&html) {
                cap[1].to_string()
            } else if let Some(cap) = re_old.captures(&html) {
                cap[1].to_string()
            } else {
                anyhow::bail!(
                    "Could not find Gemini at-token (thykhd / SNlM0e) — \
                     cookies may be expired or invalid"
                );
            }
        };

        // Build-label (bl)
        let bl = Regex::new(r#""cfb2h":"([^"]+)""#)
            .unwrap()
            .captures(&html)
            .map(|c| c[1].to_string())
            .unwrap_or_default();

        // Session ID (f.sid)
        let f_sid = Regex::new(r#""FdrFJe":"([^"]+)""#)
            .unwrap()
            .captures(&html)
            .map(|c| c[1].to_string())
            .unwrap_or_default();

        Ok(SessionTokens { at_token, f_sid, bl })
    }

    /// Build the `f.req` JSON payload — a two-element outer array whose
    /// second slot is a JSON-encoded 69-element inner array.
    fn build_freq(prompt: &str) -> String {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // 69-element flat inner list (indices 0-68)
        let mut inner: Vec<Value> = Vec::with_capacity(69);

        inner.push(json!([[prompt, 0, null, null, null, null, 0]])); // [0]
        inner.push(json!(["en"])); // [1]
        inner.push(json!([null, null, null])); // [2]
        inner.push(Value::Null); // [3]
        inner.push(Value::Null); // [4]
        inner.push(Value::Null); // [5]
        inner.push(json!([1])); // [6]
        inner.push(json!(1)); // [7]
        inner.push(Value::Null); // [8]
        inner.push(json!([1, 0, null, null, null, null, null, 0])); // [9]
        inner.push(Value::Null); // [10]
        inner.push(Value::Null); // [11]
        inner.push(json!([0])); // [12]
        for _ in 0..40 {
            inner.push(Value::Null); // [13]-[52]
        }
        inner.push(json!(0)); // [53]
        for _ in 0..5 {
            inner.push(Value::Null); // [54]-[58]
        }
        inner.push(json!("CD1035A5-0E0E-4B68-B744-23C2D8960DF5")); // [59]
        inner.push(Value::Null); // [60]
        inner.push(json!([])); // [61]
        for _ in 0..4 {
            inner.push(Value::Null); // [62]-[65]
        }
        inner.push(json!([ts, 0])); // [66]
        inner.push(Value::Null); // [67]
        inner.push(json!(2)); // [68]

        debug_assert_eq!(inner.len(), 69, "f.req inner list must be exactly 69 elements");

        let inner_json = serde_json::to_string(&inner).unwrap_or_default();
        serde_json::to_string(&json!([null, inner_json])).unwrap_or_default()
    }

    /// Walk all streaming lines and return the longest `inner[4][0][1][0]`
    /// candidate (the final, complete answer from the last streamed chunk).
    fn extract_text(raw: &str) -> String {
        let mut candidates: Vec<String> = Vec::new();

        for line in raw.lines() {
            let line = line.trim();
            if line.is_empty() || !line.starts_with('[') {
                continue;
            }
            let Ok(outer) = serde_json::from_str::<Value>(line) else {
                continue;
            };
            let Some(arr) = outer.as_array() else {
                continue;
            };
            let Some(two) = arr.get(2).and_then(Value::as_str) else {
                continue;
            };
            let Ok(inner) = serde_json::from_str::<Value>(two) else {
                continue;
            };
            if let Some(text) = inner
                .get(4)
                .and_then(|v| v.get(0))
                .and_then(|v| v.get(1))
                .and_then(|v| v.get(0))
                .and_then(Value::as_str)
            {
                candidates.push(text.to_string());
            }
        }

        candidates
            .into_iter()
            .max_by_key(|s| s.len())
            .unwrap_or_default()
    }

    /// Look up the `mode_id` string for a given model identifier.
    fn mode_id(model: &str) -> &'static str {
        MODELS
            .iter()
            .find(|(id, _, _, _)| *id == model)
            .map(|(_, mid, _, _)| *mid)
            // Default to Fast if caller passes an unrecognised model name
            .unwrap_or("fbb127bbb056c959")
    }

    /// Core request: scrape fresh tokens, build the payload, POST, parse.
    async fn ask(&self, prompt: &str, model: &str) -> Result<String> {
        let tokens = self
            .get_session_tokens()
            .await
            .context("Failed to obtain Gemini session tokens")?;

        let cookie_hdr = self.cookie_header();
        let freq = Self::build_freq(prompt);
        let mode_id = Self::mode_id(model);

        // Compact JSON for the model-selector header (no extra spaces)
        let ext_header = {
            let v: Value =
                json!([1, null, null, null, mode_id, null, null, 0, [4], null, null, 3]);
            serde_json::to_string(&v).unwrap_or_default()
        };

        // Pseudo-random request ID based on current milliseconds
        let reqid = (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
            % 900_000
            + 100_000)
            .to_string();

        let url = reqwest::Url::parse_with_params(
            GEMINI_ENDPOINT,
            &[
                ("bl", tokens.bl.as_str()),
                ("f.sid", tokens.f_sid.as_str()),
                ("hl", "en"),
                ("_reqid", reqid.as_str()),
                ("rt", "c"),
            ],
        )
        .context("Failed to build Gemini endpoint URL")?;

        let resp = self
            .client
            .post(url)
            .header("Cookie", &cookie_hdr)
            .header("X-Same-Domain", "1")
            .header("Origin", GEMINI_ORIGIN)
            .header(
                "Referer",
                format!("{}/app", GEMINI_ORIGIN),
            )
            .header("x-goog-ext-525001261-jspb", &ext_header)
            // .form() sets Content-Type: application/x-www-form-urlencoded
            .form(&[
                ("f.req", freq.as_str()),
                ("at", tokens.at_token.as_str()),
            ])
            .send()
            .await
            .context("Failed to send request to Gemini StreamGenerate")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
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
            anyhow::bail!(
                "No text found in Gemini response — raw body (first 500 chars): {}",
                &body[..body.len().min(500)]
            );
        }

        Ok(text)
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
                supports_streaming: false,
                input_cost_per_million: Some(0.0),
                output_cost_per_million: Some(0.0),
            })
            .collect())
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        // Flatten the conversation into a single prompt string
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
                        ContentPart::Text { text } => Some(text.as_str()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("");
                format!("{role}: {text}")
            })
            .collect::<Vec<_>>()
            .join("\n");

        let text = self
            .ask(&prompt, &request.model)
            .await
            .context("Gemini Web completion failed")?;

        Ok(CompletionResponse {
            message: Message {
                role: Role::Assistant,
                content: vec![ContentPart::Text { text }],
            },
            usage: Usage {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens: 0,
                cache_read_tokens: None,
                cache_write_tokens: None,
            },
            finish_reason: FinishReason::Stop,
        })
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        // Collect the full response then emit it as a single chunk
        let response = self.complete(request).await?;
        let text = response
            .message
            .content
            .iter()
            .filter_map(|p| match p {
                ContentPart::Text { text } => Some(text.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("");

        Ok(Box::pin(futures::stream::once(async move {
            StreamChunk::Text(text)
        })))
    }
}
