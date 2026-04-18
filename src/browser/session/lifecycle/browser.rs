use super::mode::{LaunchOptions, StartMode};
use crate::browser::{BrowserError, session::SessionMode};
use chromiumoxide::{
    browser::{Browser, BrowserConfig},
    handler::Handler,
};

pub(super) async fn connect_or_launch(
    mode: StartMode,
) -> Result<(Browser, Handler, SessionMode), BrowserError> {
    match mode {
        StartMode::Connect { ws_url } => connect(ws_url).await,
        StartMode::Launch(options) => launch(options).await,
    }
}

async fn connect(ws_url: String) -> Result<(Browser, Handler, SessionMode), BrowserError> {
    let (browser, handler) = Browser::connect(ws_url).await?;
    Ok((browser, handler, SessionMode::Connect))
}

async fn launch(options: LaunchOptions) -> Result<(Browser, Handler, SessionMode), BrowserError> {
    let config = config(options)?;
    let (browser, handler) = Browser::launch(config).await?;
    Ok((browser, handler, SessionMode::Launch))
}

fn config(options: LaunchOptions) -> Result<BrowserConfig, BrowserError> {
    let mut builder = BrowserConfig::builder();
    if !options.headless {
        builder = builder.with_head();
    }
    if let Some(path) = options.executable_path {
        builder = builder.chrome_executable(path);
    }
    if let Some(dir) = options.user_data_dir {
        builder = builder.user_data_dir(dir);
    }
    builder.build().map_err(BrowserError::OperationFailed)
}
