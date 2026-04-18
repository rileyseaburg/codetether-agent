use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::tool::browserctl) enum BrowserCtlAction {
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
