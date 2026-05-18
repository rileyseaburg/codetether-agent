use anyhow::Result;
use std::time::Duration;

use super::request::TetherScriptRun;
use super::run_result::TetherScriptRunResult;
use crate::tool::tetherscript::join::join_error;
use crate::tool::tetherscript::runner::{self, BrowserGrant};

/// Execute a TetherScript run on a blocking thread with a timeout.
pub async fn run(request: TetherScriptRun) -> Result<TetherScriptRunResult> {
    let timeout = Duration::from_secs(request.timeout_secs);
    let task = tokio::task::spawn_blocking(move || {
        runner::run(
            request.source_name,
            request.source,
            request.hook,
            request.args,
            BrowserGrant {
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
