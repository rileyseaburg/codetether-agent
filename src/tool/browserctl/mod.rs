use super::{Tool, ToolResult};
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

const DEFAULT_BASE_URL: &str = "http://127.0.0.1:4477";
const REQUEST_TIMEOUT_SECS: u64 = 30;

pub struct BrowserCtlTool {
    client: reqwest::Client,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum BrowserCtlAction {
    Health,
    Start,
    Stop,
    Snapshot,
    Console,
    Goto,
    Back,
    Click,
    Fill,
    Type,
    Press,
    Text,
    Html,
    Eval,
    ConsoleEval,
    ClickText,
    FillNative,
    Toggle,
    Screenshot,
    MouseClick,
    KeyboardType,
    KeyboardPress,
    Reload,
    Wait,
    Tabs,
    TabsSelect,
    TabsNew,
    TabsClose,
}

#[derive(Debug, Deserialize)]
struct BrowserCtlInput {
    action: BrowserCtlAction,
    #[serde(default)]
    base_url: Option<String>,
    #[serde(default)]
    token: Option<String>,
    #[serde(default)]
    headless: Option<bool>,
    #[serde(default)]
    executable_path: Option<String>,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    wait_until: Option<String>,
    #[serde(default)]
    selector: Option<String>,
    #[serde(default)]
    frame_selector: Option<String>,
    #[serde(default)]
    value: Option<String>,
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    text_gone: Option<String>,
    #[serde(default)]
    delay_ms: Option<u64>,
    #[serde(default)]
    key: Option<String>,
    #[serde(default)]
    expression: Option<String>,
    #[serde(default)]
    script: Option<String>,
    #[serde(default)]
    url_contains: Option<String>,
    #[serde(default)]
    state: Option<String>,
    #[serde(default)]
    timeout_ms: Option<u64>,
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    full_page: Option<bool>,
    #[serde(default)]
    x: Option<f64>,
    #[serde(default)]
    y: Option<f64>,
    #[serde(default)]
    index: Option<usize>,
    #[serde(default)]
    exact: Option<bool>,
}

impl BrowserCtlTool {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .user_agent("CodeTether-Agent/browserctl")
            .build()
            .expect("Failed to build browserctl HTTP client");
        Self { client }
    }

    fn base_url(input: &BrowserCtlInput) -> String {
        input
            .base_url
            .clone()
            .or_else(|| std::env::var("BROWSERCTL_BASE").ok())
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string())
            .trim_end_matches('/')
            .to_string()
    }

    fn token(input: &BrowserCtlInput) -> Option<String> {
        input
            .token
            .clone()
            .or_else(|| std::env::var("BROWSERCTL_TOKEN").ok())
            .filter(|value| !value.trim().is_empty())
    }

    fn require_string<'a>(value: &'a Option<String>, field: &str) -> Result<&'a str> {
        value
            .as_deref()
            .filter(|v| !v.trim().is_empty())
            .ok_or_else(|| anyhow::anyhow!("{field} is required for this browserctl action"))
    }

    fn require_index(value: Option<usize>, field: &str) -> Result<usize> {
        value.ok_or_else(|| anyhow::anyhow!("{field} is required for this browserctl action"))
    }

    fn require_point(value: Option<f64>, field: &str) -> Result<f64> {
        value.ok_or_else(|| anyhow::anyhow!("{field} is required for this browserctl action"))
    }

    fn optional_string(value: &Option<String>) -> Option<&str> {
        value.as_deref().filter(|v| !v.trim().is_empty())
    }

    async fn get(&self, base_url: &str, path: &str, token: Option<&str>) -> Result<(u16, Value)> {
        crate::tls::ensure_rustls_crypto_provider();
        let mut request = self.client.get(format!("{base_url}{path}"));
        if let Some(token) = token {
            request = request.bearer_auth(token);
        }
        let response = request.send().await.context("browserctl GET failed")?;
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        let parsed = serde_json::from_str::<Value>(&body).unwrap_or_else(|_| json!({"raw": body}));
        Ok((status, parsed))
    }

    async fn post(
        &self,
        base_url: &str,
        path: &str,
        token: Option<&str>,
        payload: Value,
    ) -> Result<(u16, Value)> {
        crate::tls::ensure_rustls_crypto_provider();
        let mut request = self.client.post(format!("{base_url}{path}")).json(&payload);
        if let Some(token) = token {
            request = request.bearer_auth(token);
        }
        let response = request.send().await.context("browserctl POST failed")?;
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        let parsed = serde_json::from_str::<Value>(&body).unwrap_or_else(|_| json!({"raw": body}));
        Ok((status, parsed))
    }

    fn response_result(
        &self,
        action: &str,
        base_url: &str,
        path: &str,
        status: u16,
        body: Value,
    ) -> Result<ToolResult> {
        let pretty = serde_json::to_string_pretty(&body).unwrap_or_else(|_| body.to_string());
        let success = (200..300).contains(&status);
        let mut metadata = HashMap::new();
        metadata.insert("action".to_string(), json!(action));
        metadata.insert("base_url".to_string(), json!(base_url));
        metadata.insert("endpoint".to_string(), json!(path));
        metadata.insert("http_status".to_string(), json!(status));
        if let Some(url) = body.get("url") {
            metadata.insert("url".to_string(), url.clone());
        }
        if let Some(title) = body.get("title") {
            metadata.insert("title".to_string(), title.clone());
        }
        if let Some(path) = body.get("path") {
            metadata.insert("path".to_string(), path.clone());
        }
        if let Some(active) = body.get("active_index") {
            metadata.insert("active_index".to_string(), active.clone());
        }
        Ok(ToolResult {
            output: pretty,
            success,
            metadata,
        })
    }

    fn screenshot_metadata(&self, body: &Value) -> Option<Value> {
        let path = body.get("path")?.as_str()?;
        let path_obj = Path::new(path);
        Some(json!({
            "path": path,
            "exists": path_obj.exists(),
            "absolute": path_obj.is_absolute(),
        }))
    }
}

#[async_trait]
impl Tool for BrowserCtlTool {
    fn id(&self) -> &str {
        "browserctl"
    }

    fn name(&self) -> &str {
        "Browser Control"
    }

    fn description(&self) -> &str {
        "Control the local browserctl shim for navigation, waiting, DOM inspection, screenshots, tabs, console evaluation, and robust interaction with modern web apps. Supports health/start/stop/snapshot/console/goto/back/click/fill/type/press/text/html/eval/console_eval/click_text/fill_native/toggle/screenshot/mouse_click/keyboard_type/keyboard_press/reload/wait/tabs/tabs_select/tabs_new/tabs_close."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": [
                        "health","start","stop","snapshot","console","goto","click","fill","type","press","text","html","eval","console_eval","click_text","fill_native","toggle","screenshot","mouse_click","keyboard_type","keyboard_press","reload","tabs","tabs_select","tabs_new","tabs_close"
                        ,"back","wait"
                    ],
                    "description": "Browserctl action to execute"
                },
                "base_url": {"type": "string", "description": "Override browserctl base URL (default: BROWSERCTL_BASE or http://127.0.0.1:4477)"},
                "token": {"type": "string", "description": "Bearer token override (default: BROWSERCTL_TOKEN env)"},
                "headless": {"type": "boolean", "description": "For start: launch headless browser (default true)"},
                "executable_path": {"type": "string", "description": "For start: optional browser binary path"},
                "url": {"type": "string", "description": "For goto or tabs_new: target URL"},
                "wait_until": {"type": "string", "description": "For goto: wait strategy, default domcontentloaded"},
                "selector": {"type": "string", "description": "CSS selector for selector-based actions"},
                "frame_selector": {"type": "string", "description": "Optional iframe selector for frame-scoped actions"},
                "value": {"type": "string", "description": "Text value for fill/fill_native"},
                "text": {"type": "string", "description": "Visible text or typed text depending on action"},
                "text_gone": {"type": "string", "description": "For wait: text that must disappear"},
                "delay_ms": {"type": "integer", "description": "For type: per-character delay in ms"},
                "key": {"type": "string", "description": "For press/keyboard_press: key to press, e.g. Enter"},
                "expression": {"type": "string", "description": "For eval: page expression to evaluate"},
                "script": {"type": "string", "description": "For console_eval: async page-side script"},
                "url_contains": {"type": "string", "description": "For wait: wait until the page URL contains this substring"},
                "state": {"type": "string", "description": "For wait: selector/text wait state, default visible"},
                "timeout_ms": {"type": "integer", "description": "For click_text/toggle: timeout in ms"},
                "path": {"type": "string", "description": "For screenshot: destination path"},
                "full_page": {"type": "boolean", "description": "For screenshot: capture full page (default true)"},
                "x": {"type": "number", "description": "For mouse_click: X coordinate"},
                "y": {"type": "number", "description": "For mouse_click: Y coordinate"},
                "index": {"type": "integer", "description": "For click_text nth match or tabs_select/tabs_close: tab index"},
                "exact": {"type": "boolean", "description": "For click_text: exact text match (default true)"}
            },
            "required": ["action"],
            "examples": [
                {"action": "health"},
                {"action": "goto", "url": "https://github.com"},
                {"action": "back"},
                {"action": "wait", "text": "Environment is ready", "timeout_ms": 15000},
                {"action": "console_eval", "script": "return { title: document.title, url: location.href };"},
                {"action": "fill_native", "selector": "#email", "value": "user@example.com"},
                {"action": "toggle", "selector": "#rating", "text": "1"},
                {"action": "screenshot", "path": "/tmp/page.png", "full_page": true}
            ]
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let input: BrowserCtlInput =
            serde_json::from_value(args).context("Invalid browserctl args")?;
        let base_url = Self::base_url(&input);
        let token = Self::token(&input);
        let token_ref = token.as_deref();

        let (action, path, status, body) = match input.action {
            BrowserCtlAction::Health => {
                let (status, body) = self.get(&base_url, "/health", token_ref).await?;
                ("health", "/health", status, body)
            }
            BrowserCtlAction::Start => {
                let payload = json!({
                    "headless": input.headless.unwrap_or(true),
                    "executable_path": input.executable_path,
                });
                let (status, body) = self.post(&base_url, "/start", token_ref, payload).await?;
                ("start", "/start", status, body)
            }
            BrowserCtlAction::Stop => {
                let (status, body) = self.post(&base_url, "/stop", token_ref, json!({})).await?;
                ("stop", "/stop", status, body)
            }
            BrowserCtlAction::Snapshot => {
                let (status, body) = self.get(&base_url, "/snapshot", token_ref).await?;
                ("snapshot", "/snapshot", status, body)
            }
            BrowserCtlAction::Console => {
                let (status, body) = self.get(&base_url, "/console", token_ref).await?;
                ("console", "/console", status, body)
            }
            BrowserCtlAction::Goto => {
                let payload = json!({
                    "url": Self::require_string(&input.url, "url")?,
                    "wait_until": input.wait_until.unwrap_or_else(|| "domcontentloaded".to_string()),
                });
                let (status, body) = self.post(&base_url, "/goto", token_ref, payload).await?;
                ("goto", "/goto", status, body)
            }
            BrowserCtlAction::Back => {
                let (status, body) = self.post(&base_url, "/back", token_ref, json!({})).await?;
                ("back", "/back", status, body)
            }
            BrowserCtlAction::Click => {
                let payload = json!({
                    "selector": Self::require_string(&input.selector, "selector")?,
                    "frame_selector": Self::optional_string(&input.frame_selector),
                });
                let (status, body) = self.post(&base_url, "/click", token_ref, payload).await?;
                ("click", "/click", status, body)
            }
            BrowserCtlAction::Fill => {
                let payload = json!({
                    "selector": Self::require_string(&input.selector, "selector")?,
                    "value": Self::require_string(&input.value, "value")?,
                    "frame_selector": Self::optional_string(&input.frame_selector),
                });
                let (status, body) = self.post(&base_url, "/fill", token_ref, payload).await?;
                ("fill", "/fill", status, body)
            }
            BrowserCtlAction::Type => {
                let payload = json!({
                    "selector": Self::require_string(&input.selector, "selector")?,
                    "text": Self::require_string(&input.text, "text")?,
                    "delay_ms": input.delay_ms.unwrap_or(0),
                    "frame_selector": Self::optional_string(&input.frame_selector),
                });
                let (status, body) = self.post(&base_url, "/type", token_ref, payload).await?;
                ("type", "/type", status, body)
            }
            BrowserCtlAction::Press => {
                let payload = json!({
                    "selector": Self::require_string(&input.selector, "selector")?,
                    "key": Self::require_string(&input.key, "key")?,
                    "frame_selector": Self::optional_string(&input.frame_selector),
                });
                let (status, body) = self.post(&base_url, "/press", token_ref, payload).await?;
                ("press", "/press", status, body)
            }
            BrowserCtlAction::Text => {
                let payload = json!({
                    "selector": Self::optional_string(&input.selector),
                    "frame_selector": Self::optional_string(&input.frame_selector),
                });
                let (status, body) = self.post(&base_url, "/text", token_ref, payload).await?;
                ("text", "/text", status, body)
            }
            BrowserCtlAction::Html => {
                let payload = json!({
                    "selector": Self::optional_string(&input.selector),
                    "frame_selector": Self::optional_string(&input.frame_selector),
                });
                let (status, body) = self.post(&base_url, "/html", token_ref, payload).await?;
                ("html", "/html", status, body)
            }
            BrowserCtlAction::Eval => {
                let payload = json!({
                    "expression": Self::require_string(&input.expression, "expression")?,
                    "frame_selector": Self::optional_string(&input.frame_selector),
                });
                let (status, body) = self.post(&base_url, "/eval", token_ref, payload).await?;
                ("eval", "/eval", status, body)
            }
            BrowserCtlAction::ConsoleEval => {
                let payload = json!({
                    "script": Self::require_string(&input.script, "script")?,
                    "frame_selector": Self::optional_string(&input.frame_selector),
                });
                let (status, body) = self
                    .post(&base_url, "/console/eval", token_ref, payload)
                    .await?;
                ("console_eval", "/console/eval", status, body)
            }
            BrowserCtlAction::ClickText => {
                let payload = json!({
                    "selector": Self::optional_string(&input.selector),
                    "frame_selector": Self::optional_string(&input.frame_selector),
                    "text": Self::require_string(&input.text, "text")?,
                    "timeout_ms": input.timeout_ms.unwrap_or(5_000),
                    "exact": input.exact.unwrap_or(true),
                    "index": input.index.unwrap_or(0),
                });
                let (status, body) = self
                    .post(&base_url, "/click/text", token_ref, payload)
                    .await?;
                ("click_text", "/click/text", status, body)
            }
            BrowserCtlAction::FillNative => {
                let payload = json!({
                    "selector": Self::require_string(&input.selector, "selector")?,
                    "value": Self::require_string(&input.value, "value")?,
                    "frame_selector": Self::optional_string(&input.frame_selector),
                });
                let (status, body) = self
                    .post(&base_url, "/fill/native", token_ref, payload)
                    .await?;
                ("fill_native", "/fill/native", status, body)
            }
            BrowserCtlAction::Toggle => {
                let payload = json!({
                    "selector": Self::require_string(&input.selector, "selector")?,
                    "text": Self::require_string(&input.text, "text")?,
                    "timeout_ms": input.timeout_ms.unwrap_or(5_000),
                    "frame_selector": Self::optional_string(&input.frame_selector),
                });
                let (status, body) = self.post(&base_url, "/toggle", token_ref, payload).await?;
                ("toggle", "/toggle", status, body)
            }
            BrowserCtlAction::Screenshot => {
                let payload = json!({
                    "path": Self::require_string(&input.path, "path")?,
                    "full_page": input.full_page.unwrap_or(true),
                    "selector": Self::optional_string(&input.selector),
                    "frame_selector": Self::optional_string(&input.frame_selector),
                });
                let (status, body) = self
                    .post(&base_url, "/screenshot", token_ref, payload)
                    .await?;
                ("screenshot", "/screenshot", status, body)
            }
            BrowserCtlAction::MouseClick => {
                let payload = json!({
                    "x": Self::require_point(input.x, "x")?,
                    "y": Self::require_point(input.y, "y")?,
                });
                let (status, body) = self
                    .post(&base_url, "/mouse/click", token_ref, payload)
                    .await?;
                ("mouse_click", "/mouse/click", status, body)
            }
            BrowserCtlAction::KeyboardType => {
                let payload = json!({"text": Self::require_string(&input.text, "text")?});
                let (status, body) = self
                    .post(&base_url, "/keyboard/type", token_ref, payload)
                    .await?;
                ("keyboard_type", "/keyboard/type", status, body)
            }
            BrowserCtlAction::KeyboardPress => {
                let payload = json!({"key": Self::require_string(&input.key, "key")?});
                let (status, body) = self
                    .post(&base_url, "/keyboard/press", token_ref, payload)
                    .await?;
                ("keyboard_press", "/keyboard/press", status, body)
            }
            BrowserCtlAction::Reload => {
                let (status, body) = self
                    .post(&base_url, "/reload", token_ref, json!({}))
                    .await?;
                ("reload", "/reload", status, body)
            }
            BrowserCtlAction::Wait => {
                let payload = json!({
                    "text": Self::optional_string(&input.text),
                    "text_gone": Self::optional_string(&input.text_gone),
                    "url_contains": Self::optional_string(&input.url_contains),
                    "selector": Self::optional_string(&input.selector),
                    "frame_selector": Self::optional_string(&input.frame_selector),
                    "state": input.state.unwrap_or_else(|| "visible".to_string()),
                    "timeout_ms": input.timeout_ms.unwrap_or(5_000),
                });
                let (status, body) = self.post(&base_url, "/wait", token_ref, payload).await?;
                ("wait", "/wait", status, body)
            }
            BrowserCtlAction::Tabs => {
                let (status, body) = self.get(&base_url, "/tabs", token_ref).await?;
                ("tabs", "/tabs", status, body)
            }
            BrowserCtlAction::TabsSelect => {
                let payload = json!({"index": Self::require_index(input.index, "index")?});
                let (status, body) = self
                    .post(&base_url, "/tabs/select", token_ref, payload)
                    .await?;
                ("tabs_select", "/tabs/select", status, body)
            }
            BrowserCtlAction::TabsNew => {
                let payload = match input.url.as_deref().filter(|s| !s.trim().is_empty()) {
                    Some(url) => json!({"url": url}),
                    None => json!({}),
                };
                let (status, body) = self
                    .post(&base_url, "/tabs/new", token_ref, payload)
                    .await?;
                ("tabs_new", "/tabs/new", status, body)
            }
            BrowserCtlAction::TabsClose => {
                let payload = match input.index {
                    Some(index) => json!({"index": index}),
                    None => json!({}),
                };
                let (status, body) = self
                    .post(&base_url, "/tabs/close", token_ref, payload)
                    .await?;
                ("tabs_close", "/tabs/close", status, body)
            }
        };

        let mut result = self.response_result(action, &base_url, path, status, body.clone())?;
        if matches!(action, "screenshot") {
            if let Some(extra) = self.screenshot_metadata(&body) {
                result = result.with_metadata("file", extra);
            }
        }
        Ok(result)
    }
}

impl Default for BrowserCtlTool {
    fn default() -> Self {
        Self::new()
    }
}
