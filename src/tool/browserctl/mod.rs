//! Browser-control tool: thin client for the local `browserctl` HTTP shim.
//!
//! The tool itself is intentionally minimal — it only owns the HTTP client
//! and the `Tool` trait impl. All real work is delegated to submodules:
//!
//! - [`input`]    — deserializable input struct + action enum
//! - [`helpers`]  — base URL, auth token, and required/optional field helpers
//! - [`http`]     — GET/POST transport
//! - [`response`] — turning `(status, body)` into a [`ToolResult`]
//! - [`schema`]   — JSON Schema advertised to the model
//! - [`dispatch`] — matches the action enum to a handler in [`actions`]
//! - [`actions`]  — per-group action handlers (nav, dom, eval, device, tabs, lifecycle)

mod actions;
mod dispatch;
mod helpers;
mod http;
mod input;
mod response;
mod schema;

use super::{Tool, ToolResult};
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde_json::Value;
use std::time::Duration;

const REQUEST_TIMEOUT_SECS: u64 = 30;

pub struct BrowserCtlTool {
    client: reqwest::Client,
}

impl BrowserCtlTool {
    pub fn new() -> Self {
        crate::tls::ensure_rustls_crypto_provider();
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .user_agent("CodeTether-Agent/browserctl")
            .build()
            .expect("Failed to build browserctl HTTP client");
        Self { client }
    }
}

impl Default for BrowserCtlTool {
    fn default() -> Self {
        Self::new()
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
        "Control the local browserctl shim for navigation, waiting, DOM inspection, screenshots, \
         tabs, console evaluation, and robust interaction with modern web apps. Supports \
         health/start/stop/snapshot/console/goto/back/click/fill/type/press/text/html/eval/\
         console_eval/click_text/fill_native/toggle/screenshot/mouse_click/keyboard_type/\
         keyboard_press/reload/wait/tabs/tabs_select/tabs_new/tabs_close."
    }

    fn parameters(&self) -> Value {
        schema::parameters_schema()
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let input: input::BrowserCtlInput =
            serde_json::from_value(args).context("Invalid browserctl args")?;
        let base_url = helpers::base_url(&input);
        let token = helpers::token(&input);
        let ctx = actions::Ctx {
            client: &self.client,
            base_url: &base_url,
            token: token.as_deref(),
            input: &input,
        };

        let (action, path, status, body) = dispatch::dispatch(&ctx).await?;
        let mut result = response::response_result(action, &base_url, path, status, body.clone())?;
        if action == "screenshot" {
            if let Some(extra) = response::screenshot_metadata(&body) {
                result = result.with_metadata("file", extra);
            }
        }
        Ok(result)
    }
}
