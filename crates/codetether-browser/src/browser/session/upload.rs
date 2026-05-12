use super::{SessionMode, access, shadow};
use crate::browser::{BrowserError, BrowserOutput, output::Ack, request::UploadRequest};
use chromiumoxide::{
    cdp::browser_protocol::dom::SetFileInputFilesParams, element::Element, page::Page,
};
use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Deserialize)]
struct UploadTarget {
    found: bool,
    multiple: bool,
    input_type: Option<String>,
    tag: String,
}

pub(super) async fn run(
    session: &crate::browser::BrowserSession,
    request: UploadRequest,
) -> Result<BrowserOutput, BrowserError> {
    page_scope(request.frame_selector.as_deref())?;
    let runtime = access::runtime(session).await?;
    let page = runtime
        .current_page
        .lock()
        .await
        .clone()
        .ok_or(BrowserError::TabClosed)?;
    let target = inspect_target(&page, &request.selector).await?;
    if !target.found {
        return Err(BrowserError::ElementNotFound(request.selector));
    }
    if target.tag != "input" || target.input_type.as_deref() != Some("file") {
        return Err(BrowserError::ElementNotFileInput {
            tag: target.tag,
            input_type: target.input_type,
        });
    }
    if request.paths.len() > 1 && !target.multiple {
        return Err(BrowserError::MultipleFilesNotAllowed(request.selector));
    }
    let files = normalize_paths(request.paths, runtime.mode)?;
    let element = resolve_element(&page, &request.selector).await?;
    page.execute(
        SetFileInputFilesParams::builder()
            .files(files)
            .object_id(element.remote_object_id.clone())
            .build()
            .map_err(BrowserError::OperationFailed)?,
    )
    .await?;
    dispatch_change(&element).await?;
    Ok(BrowserOutput::Ack(Ack { ok: true }))
}

async fn dispatch_change(element: &Element) -> Result<(), BrowserError> {
    element
        .call_js_fn(
            "function() {
                this.dispatchEvent(new Event('input', { bubbles: true }));
                this.dispatchEvent(new Event('change', { bubbles: true }));
            }",
            true,
        )
        .await?;
    Ok(())
}

fn normalize_paths(paths: Vec<String>, mode: SessionMode) -> Result<Vec<String>, BrowserError> {
    match mode {
        SessionMode::Connect => Ok(paths),
        SessionMode::Launch => paths.into_iter().map(normalize_launch_path).collect(),
    }
}

fn normalize_launch_path(path: String) -> Result<String, BrowserError> {
    let candidate = PathBuf::from(&path);
    if !candidate.exists() {
        return Err(BrowserError::FileNotFound(path));
    }
    let metadata = fs::metadata(&candidate)
        .map_err(|error| BrowserError::OperationFailed(error.to_string()))?;
    if !metadata.is_file() {
        return Err(BrowserError::OperationFailed(format!(
            "upload path is not a file: {}",
            candidate.display()
        )));
    }
    Ok(fs::canonicalize(candidate)
        .map_err(|error| BrowserError::OperationFailed(error.to_string()))?
        .to_string_lossy()
        .into_owned())
}

fn page_scope(frame_selector: Option<&str>) -> Result<(), BrowserError> {
    if frame_selector.is_some_and(|value| !value.trim().is_empty()) {
        return Err(BrowserError::OperationFailed(
            "frame-scoped upload is not implemented yet".into(),
        ));
    }
    Ok(())
}

async fn resolve_element(page: &Page, selector: &str) -> Result<Element, BrowserError> {
    page.find_element(selector)
        .await
        .map_err(|_| BrowserError::ElementNotFound(selector.to_string()))
}

async fn inspect_target(page: &Page, selector: &str) -> Result<UploadTarget, BrowserError> {
    let selector_lit = serde_json::to_string(selector)?;
    let dq = shadow::dq_call(&selector_lit);
    let script = format!(
        "(() => {{
            const el = {dq};
            if (!el) {{
                return {{ found: false, tag: '', input_type: null, multiple: false }};
            }}
            return {{
                found: true,
                tag: (el.tagName || '').toLowerCase(),
                input_type: typeof el.type === 'string' ? el.type.toLowerCase() : null,
                multiple: Boolean(el.multiple)
            }};
        }})()"
    );
    Ok(page.evaluate_expression(script).await?.into_value()?)
}
