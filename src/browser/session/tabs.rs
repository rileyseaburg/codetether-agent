use super::{BrowserSession, access};
use crate::browser::{
    BrowserError, BrowserOutput,
    output::{Ack, TabInfo, TabList},
    request::{CloseTabRequest, NewTabRequest, TabSelectRequest},
};
use chromiumoxide::cdp::browser_protocol::target::{CloseTargetParams, TargetId};
use chromiumoxide::page::Page;
use std::{sync::Arc, time::Duration};
use tokio::{sync::Mutex, time::sleep};

pub(super) async fn list(session: &BrowserSession) -> Result<BrowserOutput, BrowserError> {
    let runtime = access::runtime(session).await?;
    let current = active_target(&runtime.current_page).await;
    let pages = ordered_pages(&runtime.browser, &runtime.tab_order).await?;
    let current_index = current.and_then(|id| position(&pages, &id));
    let tabs = describe(pages).await?;
    Ok(BrowserOutput::Tabs(TabList {
        current: current_index,
        tabs,
    }))
}

pub(super) async fn select(
    session: &BrowserSession,
    request: TabSelectRequest,
) -> Result<BrowserOutput, BrowserError> {
    let runtime = access::runtime(session).await?;
    let page = page_at(&runtime.browser, &runtime.tab_order, request.index).await?;
    page.activate().await?;
    *runtime.current_page.lock().await = Some(page);
    Ok(BrowserOutput::Ack(Ack { ok: true }))
}

pub(super) async fn new(
    session: &BrowserSession,
    request: NewTabRequest,
) -> Result<BrowserOutput, BrowserError> {
    let runtime = access::runtime(session).await?;
    let page = runtime
        .browser
        .lock()
        .await
        .new_page(request.url.unwrap_or_else(|| "about:blank".into()))
        .await?;
    // Apply the stealth UA override per-page — `Network.setUserAgentOverride`
    // does not propagate to new targets, so every tab needs its own call.
    // Non-fatal: log and continue if it fails.
    if let Err(err) = super::lifecycle::apply_stealth_ua(&page).await {
        tracing::warn!(error = %err, "failed to apply stealth UA to new tab");
    }
    if let Err(err) = super::lifecycle::install_page_hooks(&page).await {
        tracing::warn!(error = %err, "failed to install page hooks on new tab");
    }
    page.activate().await?;
    runtime
        .tab_order
        .lock()
        .await
        .push(page.target_id().clone());
    *runtime.current_page.lock().await = Some(page);
    Ok(BrowserOutput::Ack(Ack { ok: true }))
}

pub(super) async fn close(
    session: &BrowserSession,
    request: CloseTabRequest,
) -> Result<BrowserOutput, BrowserError> {
    let runtime = access::runtime(session).await?;
    let current = active_target(&runtime.current_page).await;
    let pages = ordered_pages(&runtime.browser, &runtime.tab_order).await?;
    let index = request
        .index
        .or_else(|| current.as_ref().and_then(|id| position(&pages, id)))
        .ok_or(BrowserError::TabClosed)?;
    let page = pages
        .get(index)
        .cloned()
        .ok_or(BrowserError::TabNotFound(index))?;
    let target_id = page.target_id().clone();
    let was_current = current.as_ref().is_some_and(|id| id == &target_id);
    close_target(&runtime.browser, target_id.clone()).await?;
    runtime.tab_order.lock().await.retain(|id| id != &target_id);
    if was_current {
        *runtime.current_page.lock().await =
            next_page(&runtime.browser, &runtime.tab_order, index).await?;
    }
    Ok(BrowserOutput::Ack(Ack { ok: true }))
}

async fn active_target(current_page: &tokio::sync::Mutex<Option<Page>>) -> Option<TargetId> {
    current_page
        .lock()
        .await
        .as_ref()
        .map(|page| page.target_id().clone())
}

async fn describe(pages: Vec<Page>) -> Result<Vec<TabInfo>, BrowserError> {
    let mut tabs = Vec::with_capacity(pages.len());
    for (index, page) in pages.into_iter().enumerate() {
        tabs.push(TabInfo {
            index,
            url: page.url().await?.unwrap_or_default(),
            title: page.get_title().await?.unwrap_or_default(),
        });
    }
    Ok(tabs)
}

async fn next_page(
    browser: &std::sync::Arc<tokio::sync::Mutex<chromiumoxide::browser::Browser>>,
    tab_order: &tokio::sync::Mutex<Vec<TargetId>>,
    index: usize,
) -> Result<Option<Page>, BrowserError> {
    let pages = ordered_pages(browser, tab_order).await?;
    if pages.is_empty() {
        return Ok(None);
    }
    Ok(Some(pages[usize::min(index, pages.len() - 1)].clone()))
}

async fn page_at(
    browser: &std::sync::Arc<tokio::sync::Mutex<chromiumoxide::browser::Browser>>,
    tab_order: &tokio::sync::Mutex<Vec<TargetId>>,
    index: usize,
) -> Result<Page, BrowserError> {
    ordered_pages(browser, tab_order)
        .await?
        .get(index)
        .cloned()
        .ok_or(BrowserError::TabNotFound(index))
}

async fn pages(
    browser: &std::sync::Arc<tokio::sync::Mutex<chromiumoxide::browser::Browser>>,
) -> Result<Vec<Page>, BrowserError> {
    Ok(browser.lock().await.pages().await?)
}

async fn close_target(
    browser: &Arc<Mutex<chromiumoxide::browser::Browser>>,
    target_id: TargetId,
) -> Result<(), BrowserError> {
    browser
        .lock()
        .await
        .execute(CloseTargetParams::new(target_id.clone()))
        .await?;
    for _ in 0..20 {
        if pages(browser)
            .await?
            .iter()
            .all(|page| page.target_id() != &target_id)
        {
            return Ok(());
        }
        sleep(Duration::from_millis(100)).await;
    }
    Err(BrowserError::OperationFailed(format!(
        "target did not close: {target_id:?}"
    )))
}

async fn ordered_pages(
    browser: &std::sync::Arc<tokio::sync::Mutex<chromiumoxide::browser::Browser>>,
    tab_order: &tokio::sync::Mutex<Vec<TargetId>>,
) -> Result<Vec<Page>, BrowserError> {
    let mut pages = pages(browser).await?;
    let mut order = tab_order.lock().await;
    let mut ordered = Vec::with_capacity(pages.len());
    for target_id in order.iter() {
        if let Some(index) = pages.iter().position(|page| page.target_id() == target_id) {
            ordered.push(pages.remove(index));
        }
    }
    for page in pages {
        order.push(page.target_id().clone());
        ordered.push(page);
    }
    Ok(ordered)
}

fn position(pages: &[Page], target_id: &TargetId) -> Option<usize> {
    pages.iter().position(|page| page.target_id() == target_id)
}
