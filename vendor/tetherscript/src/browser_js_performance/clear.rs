//! User-timing clear methods.

use super::*;

pub(super) fn install(performance: &mut HashMap<String, JsValue>) {
    performance.insert(
        "clearMarks".into(),
        native("performance.clearMarks", None, |args| {
            state::clear("mark", optional_name(args.first()).as_deref());
            Ok(JsValue::Undefined)
        }),
    );
    performance.insert(
        "clearMeasures".into(),
        native("performance.clearMeasures", None, |args| {
            state::clear("measure", optional_name(args.first()).as_deref());
            Ok(JsValue::Undefined)
        }),
    );
}

fn optional_name(value: Option<&JsValue>) -> Option<String> {
    match value {
        Some(JsValue::Undefined | JsValue::Null) | None => None,
        Some(value) => Some(value.display()),
    }
}
