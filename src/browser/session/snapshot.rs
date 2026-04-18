use super::{BrowserSession, access};
use crate::browser::{
    BrowserError, BrowserOutput,
    output::{PageSnapshot, Viewport},
};
use serde::Deserialize;

#[derive(Deserialize)]
struct RawViewport {
    width: u32,
    height: u32,
}

pub(super) async fn run(session: &BrowserSession) -> Result<BrowserOutput, BrowserError> {
    let page = access::current_page(session).await?;
    let text = page
        .evaluate("document.body ? document.body.innerText : ''")
        .await?
        .into_value()?;
    let raw: RawViewport = page
        .evaluate("({ width: window.innerWidth, height: window.innerHeight })")
        .await?
        .into_value()?;
    let snapshot = PageSnapshot {
        url: page.url().await?.unwrap_or_default(),
        title: page.get_title().await?.unwrap_or_default(),
        text,
        viewport: Some(Viewport::from(raw)),
    };
    Ok(BrowserOutput::Snapshot(snapshot))
}

impl From<RawViewport> for Viewport {
    fn from(viewport: RawViewport) -> Self {
        Self {
            width: viewport.width,
            height: viewport.height,
        }
    }
}
