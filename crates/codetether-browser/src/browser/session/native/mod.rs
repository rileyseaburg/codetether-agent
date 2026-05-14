mod content;
mod device;
mod dispatch;
mod dom;
mod eval;
mod fetch;
mod lifecycle;
mod navigation;
mod net;
mod screen;
mod state;
mod tabs;
mod wait;

pub(super) use state::{NativePage, NativeRuntime};

use crate::browser::{BrowserCommand, BrowserError, BrowserOutput};

pub(super) async fn execute(
    session: &super::BrowserSession,
    command: BrowserCommand,
) -> Result<BrowserOutput, BrowserError> {
    dispatch::run(session, command).await
}
