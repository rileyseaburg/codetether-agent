use super::input::BrowserCtlInput;
use crate::browser::{BrowserCommand, BrowserError, BrowserOutput, browser_service};

pub(super) async fn execute(
    _input: &BrowserCtlInput,
    command: BrowserCommand,
) -> Result<BrowserOutput, BrowserError> {
    browser_service().session().execute(command).await
}

pub(super) mod device;
pub(super) mod dom;
pub(super) mod dom_extra;
pub(super) mod eval;
pub(super) mod lifecycle;
pub(super) mod nav;
pub(super) mod tabs;
