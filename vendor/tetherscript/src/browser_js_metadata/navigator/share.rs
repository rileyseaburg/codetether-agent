use super::*;
use std::cell::RefCell;
use std::rc::Rc;

const SHARE_FIELDS: [&str; 4] = ["title", "text", "url", "files"];

pub(super) fn install(
    navigator: &mut HashMap<String, JsValue>,
    host: Rc<RefCell<HashMap<String, JsValue>>>,
) {
    navigator.insert(
        "canShare".into(),
        native("navigator.canShare", None, |args| {
            Ok(JsValue::Bool(can_share(args.first())))
        }),
    );
    navigator.insert(
        "share".into(),
        native("navigator.share", None, move |args| {
            share(args, Rc::clone(&host))
        }),
    );
}

fn share(args: &[JsValue], host: Rc<RefCell<HashMap<String, JsValue>>>) -> Result<JsValue, String> {
    let data = args.first().cloned().unwrap_or(JsValue::Undefined);
    if !can_share(Some(&data)) {
        return Ok(rejection::thenable(JsValue::String(error_message())));
    }
    host.borrow_mut().insert("__lastShare".into(), copy(&data));
    Ok(thenable::resolved(JsValue::Undefined))
}

fn can_share(data: Option<&JsValue>) -> bool {
    let Some(JsValue::Object(object)) = data else {
        return false;
    };
    let object = object.borrow();
    SHARE_FIELDS.iter().any(|field| object.contains_key(*field))
}

fn copy(data: &JsValue) -> JsValue {
    match data {
        JsValue::Object(object) => JsValue::Object(Rc::new(RefCell::new(object.borrow().clone()))),
        other => other.clone(),
    }
}

fn error_message() -> String {
    "navigator.share: data is not shareable".into()
}
