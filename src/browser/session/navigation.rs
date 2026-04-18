use super::{BrowserSession, access};
use crate::browser::{BrowserError, BrowserOutput, output::Ack, request::NavigationRequest};

pub(super) async fn goto(
    session: &BrowserSession,
    request: NavigationRequest,
) -> Result<BrowserOutput, BrowserError> {
    let page = access::current_page(session).await?;
    let _ = request.wait_until;
    page.goto(request.url).await.map_err(map_error)?;
    Ok(BrowserOutput::Ack(Ack { ok: true }))
}

fn map_error(error: chromiumoxide::error::CdpError) -> BrowserError {
    match error {
        chromiumoxide::error::CdpError::Timeout => BrowserError::NavigationTimeout,
        other => other.into(),
    }
}
