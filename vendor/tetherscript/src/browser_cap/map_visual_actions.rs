//! Visual action mappings.

use crate::value::Value;

use super::super::call::BrowserCall;

pub(crate) fn selector_eval<F>(
    method: &str,
    args: &[Value],
    build: F,
) -> Result<BrowserCall, String>
where
    F: Fn(&str) -> String,
{
    Ok(super::js::eval(
        build(&super::super::args::expect_str(method, args, 0)?),
        "browser.visual",
    ))
}

pub(crate) fn screenshot_element(args: &[Value]) -> Result<BrowserCall, String> {
    Ok(BrowserCall::new(
        "screenshot",
        "browser.screenshot",
        super::super::value::map_value(vec![
            ("action", super::super::value::str_value("screenshot")),
            (
                "selector",
                super::super::args::expect_value("screenshot_element", args, 0)?,
            ),
        ]),
    ))
}

pub(crate) fn text_wait(args: &[Value]) -> Result<BrowserCall, String> {
    Ok(BrowserCall::new(
        "wait",
        "browser.visual",
        super::super::value::map_value(vec![
            ("action", super::super::value::str_value("wait")),
            (
                "text",
                super::super::value::str_value(super::super::args::expect_str(
                    "find_visual_text",
                    args,
                    0,
                )?),
            ),
        ]),
    ))
}
