mod attach;
mod browser;
mod discover;
mod handler;
mod launch;
mod mode;
mod start;
mod stop;

/// Probe the host for a Chromium-family browser executable.
///
/// Returns the absolute path of the first candidate that exists, or `None`
/// if none of the well-known install locations contain a usable binary.
pub fn detect_browser() -> Option<std::path::PathBuf> {
    discover::find_chromium_browser()
}

/// Apply the user-agent stealth override to a freshly created page so it
/// stops advertising `HeadlessChrome`. No-op if the UA is already clean.
pub(in crate::browser::session) async fn apply_stealth_ua(
    page: &chromiumoxide::page::Page,
) -> Result<(), crate::browser::BrowserError> {
    launch::apply_stealth_ua(page).await
}

/// Install the document-start hooks (network-idle tracker, webdriver
/// fingerprint removal) on a freshly created page. Errors are non-fatal.
pub(in crate::browser::session) async fn install_page_hooks(
    page: &chromiumoxide::page::Page,
) -> Result<(), crate::browser::BrowserError> {
    launch::install_page_hooks(page).await
}

pub(super) async fn start(
    session: &super::BrowserSession,
    request: crate::browser::request::StartRequest,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    start::run(session, request).await
}

pub(super) async fn stop(
    session: &super::BrowserSession,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    stop::run(session).await
}
