use crate::browser::{BrowserError, request::StartRequest};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub(super) enum StartMode {
    Launch(LaunchOptions),
    Connect { ws_url: String },
}

#[derive(Debug, Clone)]
pub(super) struct LaunchOptions {
    pub headless: bool,
    pub executable_path: Option<String>,
    pub user_data_dir: Option<PathBuf>,
}

pub(super) async fn from_request(request: StartRequest) -> Result<StartMode, BrowserError> {
    let executable_path = clean(request.executable_path);
    let user_data_dir = clean(request.user_data_dir);
    let ws_url = clean(request.ws_url);
    if let Some(raw) = ws_url {
        if executable_path.is_some() || user_data_dir.is_some() {
            return Err(BrowserError::OperationFailed(
                "ws_url cannot be combined with launch-only start fields".into(),
            ));
        }
        let resolved = resolve(&raw).await.ok_or_else(|| {
            BrowserError::OperationFailed(format!(
                "could not reach DevTools endpoint at {raw}; is the browser running \
                 with --remote-debugging-port?"
            ))
        })?;
        return Ok(StartMode::Connect { ws_url: resolved });
    }
    Ok(StartMode::Launch(LaunchOptions {
        headless: request.headless,
        executable_path,
        user_data_dir: user_data_dir.map(PathBuf::from),
    }))
}

/// Accept either a raw `ws://…/devtools/browser/<id>` URL (used as-is) or an
/// `http://host:port` base (resolved by hitting `/json/version`). This lets
/// the agent pass the friendlier `http://localhost:9222` form that a human
/// would paste from the browser's "DevTools listening on …" log line.
async fn resolve(input: &str) -> Option<String> {
    if input.starts_with("ws://") || input.starts_with("wss://") {
        return Some(input.to_string());
    }
    if input.starts_with("http://") || input.starts_with("https://") {
        return super::attach::resolve_ws_url(input).await;
    }
    None
}

fn clean(value: Option<String>) -> Option<String> {
    value.and_then(|v| (!v.trim().is_empty()).then_some(v))
}
