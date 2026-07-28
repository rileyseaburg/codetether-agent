use super::super::super::*;

pub(super) fn ns_local(args: &[JsValue]) -> (Option<String>, String) {
    (
        namespace(args.first()),
        args.get(1).unwrap_or(&JsValue::Undefined).display(),
    )
}

pub(super) fn namespace(value: Option<&JsValue>) -> Option<String> {
    match value.unwrap_or(&JsValue::Undefined) {
        JsValue::Null | JsValue::Undefined => None,
        other => {
            let text = other.display();
            (!text.is_empty()).then_some(text)
        }
    }
}

pub(super) fn local_name(qualified: &str) -> String {
    qualified
        .split_once(':')
        .map(|(_, local)| local)
        .unwrap_or(qualified)
        .into()
}
