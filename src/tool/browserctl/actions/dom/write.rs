use crate::browser::{
    BrowserCommand, BrowserError, BrowserOutput,
    request::{FillRequest, KeyPressRequest, SelectorRequest, TypeRequest},
};
use crate::tool::browserctl::helpers::require_string;
use crate::tool::browserctl::input::BrowserCtlInput;

pub(in crate::tool::browserctl) async fn click(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    let request = SelectorRequest {
        selector: require_string(&input.selector, "selector")?.to_string(),
        frame_selector: input.frame_selector.clone(),
    };
    super::super::execute(input, BrowserCommand::Click(request)).await
}

pub(in crate::tool::browserctl) async fn fill(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    let request = FillRequest {
        selector: require_string(&input.selector, "selector")?.to_string(),
        value: require_string(&input.value, "value")?.to_string(),
        frame_selector: input.frame_selector.clone(),
    };
    super::super::execute(input, BrowserCommand::Fill(request)).await
}

pub(in crate::tool::browserctl) async fn type_text(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    let request = TypeRequest {
        selector: require_string(&input.selector, "selector")?.to_string(),
        text: require_string(&input.text, "text")?.to_string(),
        delay_ms: input.delay_ms.unwrap_or(0),
        frame_selector: input.frame_selector.clone(),
    };
    super::super::execute(input, BrowserCommand::Type(request)).await
}

pub(in crate::tool::browserctl) async fn press(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    let request = KeyPressRequest {
        selector: require_string(&input.selector, "selector")?.to_string(),
        key: require_string(&input.key, "key")?.to_string(),
        frame_selector: input.frame_selector.clone(),
    };
    super::super::execute(input, BrowserCommand::Press(request)).await
}
