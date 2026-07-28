use super::*;

pub(super) fn mode(options: Option<&JsValue>) -> String {
    option_prop(options, "mode")
        .map(|value| value.display().to_ascii_lowercase())
        .unwrap_or_default()
}

pub(super) fn delegates_focus(options: Option<&JsValue>) -> bool {
    option_prop(options, "delegatesFocus").is_some_and(|value| value.truthy())
}

fn option_prop(options: Option<&JsValue>, name: &str) -> Option<JsValue> {
    options.and_then(|value| util::object_prop(value, name))
}
