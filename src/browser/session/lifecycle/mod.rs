mod browser;
mod handler;
mod launch;
mod mode;
mod start;
mod stop;

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
