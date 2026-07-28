//! Window event handler property installers.

use super::*;

const HANDLER_PROPS: &[&str] = &[
    "onpagehide",
    "onpageshow",
    "onvisibilitychange",
    "ononline",
    "onoffline",
    "onresize",
    "onscroll",
];

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    for prop in HANDLER_PROPS {
        install_one(window, prop);
    }
}

fn install_one(window: &mut HashMap<String, JsValue>, prop: &str) {
    let prop_name = prop.to_string();
    let setter_key = format!("__set:{prop}");
    let native_name = format!("set_window_{prop}");
    window.insert(prop_name.clone(), JsValue::Null);
    window.insert(
        setter_key,
        native(&native_name, Some(1), move |args| {
            let handler = args.first().cloned().unwrap_or(JsValue::Undefined);
            EVENT_REGISTRY.with(|registry| {
                registry
                    .borrow_mut()
                    .entry("window".into())
                    .or_default()
                    .handlers
                    .insert(prop_name.clone(), handler);
            });
            Ok(JsValue::Undefined)
        }),
    );
}
