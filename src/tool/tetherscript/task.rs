//! TetherScript run request and async execution wrapper.

use std::time::Duration;

use anyhow::Result;
use serde_json::Value;

use super::input::TetherScriptPluginInput;
use super::join::join_error;
use super::runner::{self, TetherScriptOutcome};

const DEFAULT_TIMEOUT_SECS: u64 = 10;
const MAX_TIMEOUT_SECS: u64 = 60;

/// Outcome of a TetherScript plugin run.
pub enum TetherScriptRunResult {
    Finished(Result<TetherScriptOutcome>),
    Timeout(u64),
}

/// Parameters for a TetherScript execution request.
pub struct TetherScriptRun {
    source_name: String,
    source: String,
    hook: String,
    args: Vec<Value>,
    timeout_secs: u64,
    grant_browser: Option<String>,
    browser_origin: Vec<String>,
    browser_scope: Vec<String>,
}

impl TetherScriptRun {
    /// Build a run request from loaded source and parsed input.
    pub fn new(source_name: String, source: String, input: TetherScriptPluginInput) -> Self {
        Self {
            source_name,
            source,
            hook: input.hook,
            args: input.args,
            timeout_secs: input
                .timeout_secs
                .unwrap_or(DEFAULT_TIMEOUT_SECS)
                .min(MAX_TIMEOUT_SECS),
            grant_browser: input.grant_browser,
            browser_origin: input.browser_origin,
            browser_scope: input.browser_scope,
        }
    }
}

/// Execute a TetherScript run on a blocking thread with a timeout.
pub async fn run(request: TetherScriptRun) -> Result<TetherScriptRunResult> {
    let timeout = Duration::from_secs(request.timeout_secs);
    let task = tokio::task::spawn_blocking(move || {
        runner::run(
            request.source_name,
            request.source,
            request.hook,
            request.args,
            runner::BrowserGrant {
                endpoint: request.grant_browser,
                origins: request.browser_origin,
                scopes: request.browser_scope,
            },
        )
    });
    match tokio::time::timeout(timeout, task).await {
        Ok(joined) => Ok(TetherScriptRunResult::Finished(joined.map_err(join_error)?)),
        Err(_) => Ok(TetherScriptRunResult::Timeout(timeout.as_secs())),
    }
}
