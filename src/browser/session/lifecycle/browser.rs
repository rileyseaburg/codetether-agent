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
    let explicit = options.executable_path.map(std::path::PathBuf::from);
    if let Some(path) = &explicit
        && !path.is_file()
    {
        return Err(BrowserError::OperationFailed(format!(
            "executable_path does not exist or is not a file: {}",
            path.display()
        )));
    }
    let resolved = explicit.or_else(super::discover::find_chromium_browser);
    match resolved {
        Some(path) => {
            tracing::info!(executable = %path.display(), "launching chromium-family browser");
            builder = builder.chrome_executable(path);
        }
        None => {
            return Err(BrowserError::OperationFailed(no_browser_error()));
        }
    }
    if let Some(dir) = options.user_data_dir {
        builder = builder.user_data_dir(dir);
    }

    // Try to bind the DevTools Protocol on the first free well-known port
    // (9222, then 9223). The next `start` call will find this port open via
    // [`super::attach::detect_running_browser`] and **attach** rather than
    // spawning another copy that fights over the profile lock. If neither
    // port is free, chromiumoxide falls back to an ephemeral port — we still
    // work, we just lose the attach-on-reconnect benefit for this session.
    if let Some(port) = super::attach::pick_free_port() {
        builder = builder.port(port);
    }

    // Replace chromiumoxide's default args with an automation-free equivalent
    // so the user / viewer never sees "Chrome is being controlled by automated
    // test software" and `navigator.webdriver` is not exposed.
    builder = builder.disable_default_args().args(stealth_args());

    builder.build().map_err(BrowserError::OperationFailed)
}

/// Chromium launch flags mirroring chromiumoxide's default set **minus**
/// `--enable-automation`, plus flags that suppress the automation indicator
/// banner and the `navigator.webdriver` property.
fn stealth_args() -> Vec<&'static str> {
    vec![
        // Useful defaults preserved from chromiumoxide.
        "--disable-background-networking",
        "--enable-features=NetworkService,NetworkServiceInProcess",
        "--disable-background-timer-throttling",
        "--disable-backgrounding-occluded-windows",
        "--disable-breakpad",
        "--disable-client-side-phishing-detection",
        "--disable-component-extensions-with-background-pages",
        "--disable-default-apps",
        "--disable-dev-shm-usage",
        "--disable-hang-monitor",
        "--disable-ipc-flooding-protection",
        "--disable-popup-blocking",
        "--disable-prompt-on-repost",
        "--disable-renderer-backgrounding",
        "--disable-sync",
        "--force-color-profile=srgb",
        "--metrics-recording-only",
        "--no-first-run",
        "--no-default-browser-check",
        "--password-store=basic",
        "--use-mock-keychain",
        "--lang=en_US",
        // Realistic laptop-class window. Headless mode otherwise defaults to
        // 800x600 which is a huge fingerprint tell.
        "--window-size=1280,800",
        // Stealth: hide the fact that this is an automated session.
        // Combined: TranslateUI (from defaults) + AutomationControlled (stealth).
        "--disable-features=TranslateUI,AutomationControlled",
        "--disable-blink-features=AutomationControlled",
        // IdleDetection kept from defaults.
        "--enable-blink-features=IdleDetection",
        // NOTE: deliberately omitting `--enable-automation` — its presence is
        // what makes Chrome render the "controlled by automated test software"
        // infobar and expose `navigator.webdriver = true`.
    ]
}

#[cfg(target_os = "windows")]
fn no_browser_error() -> String {
    "No Chromium-family browser found. Install Chrome, Edge, Brave, or Vivaldi, \
     or pass an `executable_path` to the `start` action (e.g. \
     \"C:\\\\Program Files (x86)\\\\Microsoft\\\\Edge\\\\Application\\\\msedge.exe\"). \
     Firefox and Safari are not supported — this tool uses the Chrome DevTools Protocol."
        .into()
}

#[cfg(target_os = "macos")]
fn no_browser_error() -> String {
    "No Chromium-family browser found. Install Chrome, Edge, Brave, or Vivaldi from \
     /Applications, or pass an `executable_path` to the `start` action. \
     Firefox and Safari are not supported — this tool uses the Chrome DevTools Protocol."
        .into()
}

#[cfg(all(unix, not(target_os = "macos")))]
fn no_browser_error() -> String {
    "No Chromium-family browser found. Install one of: google-chrome, chromium, \
     microsoft-edge, brave-browser, vivaldi — or pass an `executable_path` to the \
     `start` action. Firefox is not supported — this tool uses the Chrome DevTools Protocol."
        .into()
}
