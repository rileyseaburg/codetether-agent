//! Compound DOM action mappings.

use crate::value::Value;

use super::super::call::BrowserCall;

pub(crate) fn upload(args: &[Value]) -> Result<BrowserCall, String> {
    let selector = super::super::args::expect_str("upload", args, 0)?;
    Ok(super::basic::call(
        "upload",
        vec![
            ("selector", super::super::value::str_value(selector)),
            (
                "paths",
                super::super::args::expect_value("upload", args, 1)?,
            ),
        ],
    ))
}

pub(crate) fn click_text(args: &[Value]) -> Result<BrowserCall, String> {
    Ok(super::basic::call(
        "click_text",
        vec![
            (
                "text",
                super::super::value::str_value(super::super::args::expect_str(
                    "click_text",
                    args,
                    0,
                )?),
            ),
            (
                "timeout_ms",
                Value::Int(super::super::args::optional_int(args, 1, 30_000)?),
            ),
        ],
    ))
}

pub(crate) fn xy(args: &[Value]) -> Result<BrowserCall, String> {
    Ok(super::basic::call(
        "mouse_click",
        vec![
            (
                "x",
                super::super::args::expect_value("mouse_click", args, 0)?,
            ),
            (
                "y",
                super::super::args::expect_value("mouse_click", args, 1)?,
            ),
        ],
    ))
}
