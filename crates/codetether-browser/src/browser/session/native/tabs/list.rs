use crate::browser::{
    BrowserError, BrowserOutput,
    output::{TabInfo, TabList},
};

pub(super) async fn list(
    session: &super::super::super::BrowserSession,
) -> Result<BrowserOutput, BrowserError> {
    let native = session.inner.native.lock().await;
    let runtime = native.as_ref().ok_or(BrowserError::SessionNotStarted)?;
    let tabs = runtime
        .pages
        .iter()
        .enumerate()
        .map(|(index, page)| TabInfo {
            index,
            url: page.session.url.clone(),
            title: super::super::content::title(page),
        })
        .collect();
    Ok(BrowserOutput::Tabs(TabList {
        current: (!runtime.pages.is_empty()).then_some(runtime.current),
        tabs,
    }))
}
