//! Browserctl upload action adapter.

use super::super::input::BrowserCtlInput;
use crate::browser::{BrowserCommand, BrowserError, BrowserOutput, request::UploadRequest};
use crate::tool::browserctl::helpers::require_string;

/// Build and execute a file upload command.
///
/// # Errors
///
/// Returns [`BrowserError`] when `selector`, `paths`, or execution fails.
pub(in crate::tool::browserctl) async fn upload(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    let request = UploadRequest {
        selector: require_string(&input.selector, "selector")?.to_string(),
        paths: paths(input)?,
        frame_selector: input.frame_selector.clone(),
    };
    super::execute(input, BrowserCommand::Upload(request)).await
}

fn paths(input: &BrowserCtlInput) -> Result<Vec<String>, BrowserError> {
    let mut paths = input.paths.clone().unwrap_or_default();
    if let Some(path) = input.path.as_ref().filter(|value| !value.trim().is_empty()) {
        paths.push(path.clone());
    }
    paths.retain(|value| !value.trim().is_empty());
    if paths.is_empty() {
        return Err(BrowserError::OperationFailed(
            "upload requires path or paths".into(),
        ));
    }
    Ok(paths)
}
