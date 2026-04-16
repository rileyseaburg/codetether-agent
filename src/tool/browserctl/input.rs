//! Input types for the browserctl tool.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum BrowserCtlAction {
    Health,
    Start,
    Stop,
    Snapshot,
    Console,
    Goto,
    Back,
    Click,
    Fill,
    Type,
    Press,
    Text,
    Html,
    Eval,
    ConsoleEval,
    ClickText,
    FillNative,
    Toggle,
    Screenshot,
    MouseClick,
    KeyboardType,
    KeyboardPress,
    Reload,
    Wait,
    Tabs,
    TabsSelect,
    TabsNew,
    TabsClose,
}

#[derive(Debug, Deserialize)]
pub(super) struct BrowserCtlInput {
    pub action: BrowserCtlAction,
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default)]
    pub headless: Option<bool>,
    #[serde(default)]
    pub executable_path: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub wait_until: Option<String>,
    #[serde(default)]
    pub selector: Option<String>,
    #[serde(default)]
    pub frame_selector: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub text_gone: Option<String>,
    #[serde(default)]
    pub delay_ms: Option<u64>,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub expression: Option<String>,
    #[serde(default)]
    pub script: Option<String>,
    #[serde(default)]
    pub url_contains: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub timeout_ms: Option<u64>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub full_page: Option<bool>,
    #[serde(default)]
    pub x: Option<f64>,
    #[serde(default)]
    pub y: Option<f64>,
    #[serde(default)]
    pub index: Option<usize>,
    #[serde(default)]
    pub exact: Option<bool>,
}
