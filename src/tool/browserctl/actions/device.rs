use super::super::helpers::{require_point, require_string};
use super::super::input::BrowserCtlInput;
use crate::browser::{
    BrowserCommand,
    request::{KeyboardPressRequest, KeyboardTypeRequest, PointerClick},
};

pub(in crate::tool::browserctl) async fn mouse_click(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    let request = PointerClick {
        x: require_point(input.x, "x")?,
        y: require_point(input.y, "y")?,
    };
    super::execute(input, BrowserCommand::MouseClick(request)).await
}

pub(in crate::tool::browserctl) async fn keyboard_type(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    let request = KeyboardTypeRequest {
        text: require_string(&input.text, "text")?.to_string(),
    };
    super::execute(input, BrowserCommand::KeyboardType(request)).await
}

pub(in crate::tool::browserctl) async fn keyboard_press(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    let request = KeyboardPressRequest {
        key: require_string(&input.key, "key")?.to_string(),
    };
    super::execute(input, BrowserCommand::KeyboardPress(request)).await
}
