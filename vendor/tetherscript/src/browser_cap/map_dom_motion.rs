//! Pointer and viewport movement mappings.

use crate::value::Value;

use super::super::call::BrowserCall;

pub(crate) fn scroll(args: &[Value]) -> Result<BrowserCall, String> {
    match args {
        [Value::Str(_)] => super::basic::selector_action("scroll", args),
        [_, _] => xy(args, 0),
        [Value::Str(_), _, _] => selector_xy(args),
        _ => Err("browser.scroll expects selector, x/y, or selector plus x/y".into()),
    }
}

fn xy(args: &[Value], offset: usize) -> Result<BrowserCall, String> {
    Ok(super::basic::call(
        "scroll",
        vec![
            (
                "x",
                Value::Int(super::super::args::expect_int("scroll", args, offset)?),
            ),
            (
                "y",
                Value::Int(super::super::args::expect_int("scroll", args, offset + 1)?),
            ),
        ],
    ))
}

fn selector_xy(args: &[Value]) -> Result<BrowserCall, String> {
    Ok(super::basic::call(
        "scroll",
        vec![
            (
                "selector",
                super::super::value::str_value(super::super::args::expect_str("scroll", args, 0)?),
            ),
            (
                "x",
                Value::Int(super::super::args::expect_int("scroll", args, 1)?),
            ),
            (
                "y",
                Value::Int(super::super::args::expect_int("scroll", args, 2)?),
            ),
        ],
    ))
}
