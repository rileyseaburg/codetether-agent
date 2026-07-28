//! User-timing timeline read methods.

use super::*;

pub(super) fn install(performance: &mut HashMap<String, JsValue>) {
    performance.insert(
        "getEntries".into(),
        native("performance.getEntries", Some(0), |_| {
            Ok(entry::array(state::entries()))
        }),
    );
    performance.insert(
        "getEntriesByType".into(),
        native("performance.getEntriesByType", Some(1), |args| {
            Ok(entry::array(state::by_type(&args[0].display())))
        }),
    );
    performance.insert(
        "getEntriesByName".into(),
        native("performance.getEntriesByName", None, |args| {
            Ok(entry::array(state::by_name(
                &args.first().map(JsValue::display).unwrap_or_default(),
                entry_type(args.get(1)).as_deref(),
            )))
        }),
    );
}

fn entry_type(value: Option<&JsValue>) -> Option<String> {
    match value {
        Some(JsValue::Undefined | JsValue::Null) | None => None,
        Some(value) => Some(value.display()),
    }
}
