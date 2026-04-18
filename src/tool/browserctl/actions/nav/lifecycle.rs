use crate::browser::{BrowserCommand, BrowserError, BrowserOutput, request::StartRequest};
use crate::tool::browserctl::input::BrowserCtlInput;

pub(in crate::tool::browserctl) async fn health(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    super::super::execute(input, BrowserCommand::Health).await
}

pub(in crate::tool::browserctl) async fn start(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    let request = StartRequest {
        headless: input.headless.unwrap_or(true),
        executable_path: input.executable_path.clone(),
        user_data_dir: input.user_data_dir.clone(),
        ws_url: input.ws_url.clone(),
    };
    super::super::execute(input, BrowserCommand::Start(request)).await
}

pub(in crate::tool::browserctl) async fn stop(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    super::super::execute(input, BrowserCommand::Stop).await
}

pub(in crate::tool::browserctl) async fn snapshot(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    super::super::execute(input, BrowserCommand::Snapshot).await
}

pub(in crate::tool::browserctl) async fn console(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    super::super::execute(input, BrowserCommand::Console).await
}
