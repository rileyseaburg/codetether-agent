use super::{BrowserSession, access};
use crate::browser::{
    BrowserError, BrowserOutput, output::ScreenshotData, request::ScreenshotRequest,
};
use chromiumoxide::page::ScreenshotParams;

pub(super) async fn capture(
    session: &BrowserSession,
    request: ScreenshotRequest,
) -> Result<BrowserOutput, BrowserError> {
    if request.selector.is_some() || request.frame_selector.is_some() {
        return Err(BrowserError::OperationFailed(
            "selector screenshots are not implemented yet".into(),
        ));
    }
    let page = access::current_page(session).await?;
    let params = ScreenshotParams::builder()
        .full_page(request.full_page)
        .build();
    let bytes = page.screenshot(params).await?;
    Ok(BrowserOutput::Screenshot(ScreenshotData { bytes }))
}
