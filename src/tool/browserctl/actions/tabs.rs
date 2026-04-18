use super::super::helpers::require_index;
use super::super::input::BrowserCtlInput;
use crate::browser::{
    BrowserCommand,
    request::{CloseTabRequest, NewTabRequest, TabSelectRequest},
};

pub(in crate::tool::browserctl) async fn tabs(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    super::execute(input, BrowserCommand::Tabs).await
}

pub(in crate::tool::browserctl) async fn tabs_select(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    super::execute(
        input,
        BrowserCommand::TabsSelect(TabSelectRequest {
            index: require_index(input.index, "index")?,
        }),
    )
    .await
}

pub(in crate::tool::browserctl) async fn tabs_new(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    super::execute(
        input,
        BrowserCommand::TabsNew(NewTabRequest {
            url: input.url.clone().filter(|value| !value.trim().is_empty()),
        }),
    )
    .await
}

pub(in crate::tool::browserctl) async fn tabs_close(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    super::execute(
        input,
        BrowserCommand::TabsClose(CloseTabRequest { index: input.index }),
    )
    .await
}
