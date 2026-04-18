use crate::browser::{BrowserCommand, BrowserError, BrowserOutput, request::ScopeRequest};
use crate::tool::browserctl::helpers::optional_string;
use crate::tool::browserctl::input::BrowserCtlInput;

pub(in crate::tool::browserctl) async fn text(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    let request = ScopeRequest {
        selector: optional_string(&input.selector).map(str::to_string),
        frame_selector: optional_string(&input.frame_selector).map(str::to_string),
    };
    super::super::execute(input, BrowserCommand::Text(request)).await
}

pub(in crate::tool::browserctl) async fn html(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    let request = ScopeRequest {
        selector: optional_string(&input.selector).map(str::to_string),
        frame_selector: optional_string(&input.frame_selector).map(str::to_string),
    };
    super::super::execute(input, BrowserCommand::Html(request)).await
}
