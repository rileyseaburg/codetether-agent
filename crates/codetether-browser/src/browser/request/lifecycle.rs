use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct StartRequest {
    pub headless: bool,
    pub executable_path: Option<String>,
    pub user_data_dir: Option<String>,
    pub ws_url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WaitRequest {
    pub text: Option<String>,
    pub text_gone: Option<String>,
    pub url_contains: Option<String>,
    pub selector: Option<String>,
    pub frame_selector: Option<String>,
    pub state: String,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScreenshotRequest {
    pub selector: Option<String>,
    pub frame_selector: Option<String>,
    pub full_page: bool,
}
