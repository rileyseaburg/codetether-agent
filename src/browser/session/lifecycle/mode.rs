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

pub(super) fn from_request(request: StartRequest) -> Result<StartMode, BrowserError> {
    let executable_path = clean(request.executable_path);
    let user_data_dir = clean(request.user_data_dir);
    let ws_url = clean(request.ws_url);
    if let Some(ws_url) = ws_url {
        if executable_path.is_some() || user_data_dir.is_some() {
            return Err(BrowserError::OperationFailed(
                "ws_url cannot be combined with launch-only start fields".into(),
            ));
        }
        return Ok(StartMode::Connect { ws_url });
    }
    Ok(StartMode::Launch(LaunchOptions {
        headless: request.headless,
        executable_path,
        user_data_dir: user_data_dir.map(PathBuf::from),
    }))
}

fn clean(value: Option<String>) -> Option<String> {
    value.and_then(|v| (!v.trim().is_empty()).then_some(v))
}
