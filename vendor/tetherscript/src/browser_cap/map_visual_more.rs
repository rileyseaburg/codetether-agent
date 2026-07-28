//! Additional visual action mappings.

use crate::value::Value;

use super::super::call::BrowserCall;

pub(crate) fn point_eval(args: &[Value]) -> Result<BrowserCall, String> {
    let x = super::super::args::expect_int("find_element_at", args, 0)?;
    let y = super::super::args::expect_int("find_element_at", args, 1)?;
    Ok(super::js::eval(
        format!("document.elementFromPoint({x},{y})?.outerHTML || null"),
        "browser.visual",
    ))
}

pub(crate) fn raw_visual(method: &str, _: &[Value]) -> Result<BrowserCall, String> {
    Err(format!(
        "browser.{}: browserctl backend does not support visual diff actions",
        method
    ))
}
